lib = File.expand_path("lib", __dir__)
$LOAD_PATH.unshift lib unless $LOAD_PATH.include?(lib)

Gem::Specification.new do |s|
  s.name = "typst"
  s.version = "0.13.5"
  s.authors = "Flinn"
  s.email = "flinn@actsasflinn.com"
  s.files = Dir["{lib}/**/*.{rb,ttf,otf,so,bundle}", "ext/**/*.{rs,toml,lock,rb,ttf}"] + %w[README.md README.typ Cargo.toml Rakefile]
  s.homepage = "https://github.com/actsasflinn/typst-rb"
  s.require_paths = ["lib"]
  s.extensions = %w[ext/typst/extconf.rb]
  s.summary = "Ruby binding to typst, a new markup-based typesetting system that is powerful and easy to learn."
  s.license = "Apache-2.0"
  s.required_ruby_version = ">= 3.0.0"

  s.add_dependency "rb_sys", ">= 0.9.116"
  s.add_dependency "rubyzip", "~> 3.2"
  s.add_development_dependency "hexapdf", "~> 1.3"
  s.add_development_dependency "test-unit", "~> 3.6"
end
