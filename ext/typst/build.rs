fn main() {
    // Redirect every `Sleep` call in the extension to `__wrap_Sleep`.
    //
    // The Ruby DLL exports a symbol named `Sleep`: it is `rb_w32_Sleep` from
    // `thread_win32.c`, and its whole body is a `BLOCKING_REGION`, so it
    // releases and reacquires the GVL. rb_sys names the Ruby import library on
    // the link line ahead of the system ones, and ld binds an undefined symbol
    // to the first library that offers it, so `Sleep` resolves to Ruby's rather
    // than to the Win32 call - see `src/nogvl.rs` for what that costs once the
    // GVL has been released.
    //
    // Neither link order nor a plain definition can settle this: rb_sys appends
    // libruby after anything a build script or RUSTFLAGS can add, and both
    // import libraries define `Sleep`, so defining it here as well only earns a
    // multiple-definition error. `--wrap` is order-independent - it rewrites
    // the references themselves.
    //
    // `Sleep` alone is not enough. The Win32 headers declare it `WINBASEAPI`,
    // that is `__declspec(dllimport)`, so C objects reference the `__imp_Sleep`
    // thunk rather than the plain symbol - mingw's own `_CRT_INIT` does, in the
    // `__native_startup_lock` spin loop it runs from `DllMain`. `--wrap` is a
    // name rewrite and does not relate the two spellings, so the thunk needs
    // its own wrap and its own definition in `src/nogvl.rs`.
    //
    // It is also a GNU ld option, so this is limited to the GNU toolchain,
    // which is the one the published Windows binaries are built with
    // (`x64-mingw-ucrt`). An MSVC build links `Sleep` the same way and would
    // need its own answer.
    let windows = std::env::var("CARGO_CFG_WINDOWS").is_ok();
    let gnu = std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu");

    if windows && gnu {
        println!("cargo:rustc-link-arg=-Wl,--wrap=Sleep");
        println!("cargo:rustc-link-arg=-Wl,--wrap=__imp_Sleep");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
