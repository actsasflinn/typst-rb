require "mkmf"
require "rb_sys/mkmf"

# On Windows platforms kernel32 should come before libruby in ld flags. The typst crate comemo depends
# on the parking_lot crate which binds to kernel32's Sleep function. Ruby exports functions with names that
# overlap with Windows kernel32 functions which causes a bug. There doesn't seem to be a more orthodox way of
# injecting libraries within rb_sys. Ruby's mkmf.rb has append_library which seems to provide this functionality
# for non-Rust but doesn't work within rb_sys. rb_sys's CargoBuilder uses RbConfig::CONFIG["LIBS"] in
# rustc_lib_flags called platform_specific_rustc_args which adds libruby for mingw platform.
RbConfig::CONFIG["LIBS"] << " -lkernel32" if !!Gem::WIN_PATTERNS.find { |r| RbConfig::CONFIG["target_os"] =~ r }

create_rust_makefile("typst/typst")
