require "bundler/gem_tasks"
require "rake/extensiontask"
require "rake/testtask"
require "rubygems/package_task"
require "bundler"

CROSS_PLATFORMS = %w[
  aarch64-linux
  arm64-darwin
  x64-mingw32
  x86_64-darwin
  x86_64-linux
  x86_64-linux-musl
]

spec = Bundler.load_gemspec("typst.gemspec")

Gem::PackageTask.new(spec).define

Rake::ExtensionTask.new("typst", spec) do |ext|
  ext.lib_dir = "lib/typst"
  ext.source_pattern = "*.{rs,toml}"
  ext.cross_compile = true
  ext.cross_platform = CROSS_PLATFORMS
end

Rake::TestTask.new do |t|
  t.libs << "test"
  t.test_files = FileList['test/*_test.rb']
  t.verbose = true
end