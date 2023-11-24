spec.extensions = ["ext/typst/extconf.rb"]

# needed until rubygems supports Rust support is out of beta
spec.add_dependency "rb_sys", "~> 0.9.39"

# only needed when developing or packaging your gem
spec.add_development_dependency "rake-compiler", "~> 1.2.0"
