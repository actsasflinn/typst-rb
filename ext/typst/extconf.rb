require "mkmf"
require "rb_sys/mkmf"

module RbSys
    class CargoBuilder < Gem::Ext::Builder
        alias_method :old_platform_specific_rustc_args, :platform_specific_rustc_args
        def new_platform_specific_rustc_args(dest_dir, flags = [])
            flags += ["-l", "kernel32"] if mingw_target? # add kernel32 before libruby to avoid the sleep bug
            old_platform_specific_rustc_args(dest_dir, flags)
        end
        alias_method :platform_specific_rustc_args, :new_platform_specific_rustc_args
    end
end

create_rust_makefile("typst/typst")
