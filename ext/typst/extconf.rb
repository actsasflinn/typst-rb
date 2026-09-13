require "mkmf"
require "rb_sys/mkmf"

module RbSys
    class CargoBuilder < Gem::Ext::Builder
        def platform_specific_rustc_args(dest_dir, flags = [])
            if mingw_target?
            # On mingw platforms, mkmf adds libruby to the linker flags
            flags += ["-l", "kernel32"]
            flags += libruby_args(dest_dir)

            # Make sure ALSR is used on mingw
            # see https://github.com/rust-lang/rust/pull/75406/files
            flags += ["-C", "link-arg=-Wl,--dynamicbase"]
            flags += ["-C", "link-arg=-Wl,--disable-auto-image-base"]

            # If the gem is installed on a host with build tools installed, but is
            # run on one that isn't the missing libraries will cause the extension
            # to fail on start.
            flags += ["-C", "link-arg=-static-libgcc"]
            elsif darwin_target?
            # See https://github.com/oxidize-rb/rb-sys/issues/88
            dl_flag = "-Wl,-undefined,dynamic_lookup"
            flags += ["-C", "link-arg=#{dl_flag}"] unless makefile_config("DLDFLAGS")&.include?(dl_flag)
            elsif RUBY_ENGINE == "truffleruby"
            dl_flag = "-Wl,-z,lazy"
            flags += ["-C", "link-arg=#{dl_flag}"] unless makefile_config("DLDFLAGS")&.include?(dl_flag)
            # lazy binding requires RELRO to be off, see https://users.rust-lang.org/t/linux-executable-lazy-loading/65621/2
            flags += ["-C", "relro-level=off"]
            end

            if musl?
            flags += ["-C", "target-feature=-crt-static"]
            end

            flags
        end
    end
end

create_rust_makefile("typst/typst")
