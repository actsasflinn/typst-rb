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
