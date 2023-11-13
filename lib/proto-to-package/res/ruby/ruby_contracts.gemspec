# frozen_string_literal: true

require_relative "lib/ruby_contracts/version"

Gem::Specification.new do |spec|
  spec.name = "${package_name}"
  spec.version = RubyContracts::VERSION
  spec.authors = ["${package_author}"]
  spec.email = ["${package_author_email}"]

  spec.summary = "${package_name}"
  spec.description = "${package_name} gem with grpc and protobuf compiled ruby objects"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.6.0"

#   spec.metadata["allowed_push_host"] = "TODO: Set to your gem server 'https://example.com'"

#   spec.metadata["homepage_uri"] = spec.homepage
#   spec.metadata["source_code_uri"] = "TODO: Put your gem's public repo URL here."
#   spec.metadata["changelog_uri"] = "TODO: Put your gem's CHANGELOG.md URL here."

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) || f.start_with?(*%w[bin/ test/ spec/ features/ .git .circleci appveyor])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  # Uncomment to register a new dependency of your gem
  spec.add_dependency 'grpc', '~> 1.59'
  spec.add_dependency "google-protobuf", "~> 3.25"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
