require "bundler/gem_tasks"
require "rb_sys/extensiontask"
require "rake/testtask"

CROSS_PLATFORMS = %w[
  aarch64-linux
  aarch64-linux-musl
  arm64-darwin
  x64-mingw-ucrt
  x86_64-darwin
  x86_64-linux
  x86_64-linux-musl
]

spec = Bundler.load_gemspec("typst.gemspec")

Gem::PackageTask.new(spec).define

RbSys::ExtensionTask.new("typst", spec) do |ext|
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

task 'gem:native' do |t|
  CROSS_PLATFORMS.each do |platform|
    sh "bundle exec rb-sys-dock --platform #{platform} --build"
  end
end

task 'gem:native:push' do |t|
  sh "gem signin"
  CROSS_PLATFORMS.each do |platform|
    sh "gem push pkg/typst-#{spec.version}-#{platform}.gem"
  end
end