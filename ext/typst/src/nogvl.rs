use std::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};

/// Runs `f` with the GVL released, so other Ruby threads keep running while
/// Typst compiles.
///
/// `f` runs on the calling OS thread and must not touch the Ruby API — no
/// `Ruby` handle, no `Value`, no allocation of Ruby objects. Convert to and
/// from Ruby types outside of this call.
///
/// No unblocking function is registered, so the call cannot be interrupted;
/// a compile always runs to completion. That matches the previous behaviour,
/// where holding the GVL made the compile uninterruptible anyway.
pub fn without_gvl<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    struct Payload<F, T> {
        f: Option<F>,
        result: Option<std::thread::Result<T>>,
    }

    // A Rust panic must not unwind across the C frame that Ruby pushes here,
    // so it is caught and resumed once we hold the GVL again.
    unsafe extern "C" fn trampoline<F, T>(data: *mut c_void) -> *mut c_void
    where
        F: FnOnce() -> T,
    {
        let payload = &mut *data.cast::<Payload<F, T>>();
        let f = payload.f.take().expect("closure called twice");
        payload.result = Some(catch_unwind(AssertUnwindSafe(f)));
        std::ptr::null_mut()
    }

    let mut payload = Payload { f: Some(f), result: None };
    unsafe {
        rb_sys::rb_thread_call_without_gvl(
            Some(trampoline::<F, T>),
            (&raw mut payload).cast::<c_void>(),
            None,
            std::ptr::null_mut(),
        );
    }

    match payload.result.take().expect("closure was not called") {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

/// Stands in for Windows' `Sleep` throughout the extension.
///
/// `build.rs` links with `--wrap=Sleep`, so every call that would otherwise
/// bind to a `Sleep` import arrives here instead - and with
/// `--wrap=__imp_Sleep` for the dllimport spelling, see below. That matters because the Ruby
/// DLL exports one: `rb_w32_Sleep` from `thread_win32.c`, whose whole body is a
/// `BLOCKING_REGION`, so it releases and reacquires the GVL. It is the first
/// library on the link line to offer the symbol, so without the wrap it is the
/// one `parking_lot` reaches while spinning for a contended lock - which a
/// compile gets to through comemo's cache.
///
/// Called from a thread that has already released the GVL, it releases a mutex
/// the thread does not own - silently, because Ruby ignores what `ReleaseMutex`
/// returns - and then takes the GVL for a thread that is meant to be outside
/// it. The compile runs on holding the GVL while every other thread waits, and
/// Ruby's own `blocking_region_end` later locks the same recursive Win32 mutex
/// a second time. Its count never returns to zero, so the mutex is abandoned
/// when the thread exits, and the next waiter dies with
/// `[BUG] win32_mutex_lock: WAIT_ABANDONED`.
///
/// `SleepEx(ms, FALSE)` is the same wait without the alertable part, and Ruby
/// exports no symbol by that name.
#[cfg(windows)]
#[no_mangle]
pub extern "system" fn __wrap_Sleep(milliseconds: u32) {
    extern "system" {
        fn SleepEx(milliseconds: u32, alertable: i32) -> u32;
    }

    unsafe { SleepEx(milliseconds, 0) };
}

/// The dllimport spelling of the same wrap.
///
/// `Sleep` is `WINBASEAPI`, so C code reaches it through this pointer rather
/// than through the symbol: `mov __imp_Sleep(%rip), %reg; call *%reg`. `--wrap`
/// rewrites names and knows nothing of that relationship, so `__imp_Sleep` gets
/// its own wrap in `build.rs` and lands here.
///
/// The only object that asks for it is mingw's `_CRT_INIT`, which backs off
/// with `Sleep(1000)` while `__native_startup_lock` is contended. It runs from
/// `DllMain` on a thread that still holds the GVL and is serialised by the
/// loader lock anyway, so it is not the path that crashed - but wrapping it too
/// leaves one invariant with no exceptions to remember: the extension imports
/// no `Sleep` from the Ruby DLL at all.
#[cfg(windows)]
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static __wrap___imp_Sleep: extern "system" fn(u32) = __wrap_Sleep;
