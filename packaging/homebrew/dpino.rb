class Dpino < Formula
  desc "Rust-powered package metadata extractor + desktop entry installer + browser"
  homepage "https://github.com/abdou-da0wew/dpino"
  url "https://github.com/abdou-da0wew/dpino/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "SKIP"
  license "MIT OR Apache-2.0"
  head "https://github.com/abdou-da0wew/dpino.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    bin.install "target/release/dpino" if File.exist?("target/release/dpino")
  end

  test do
    system "#{bin}/dpino", "--version"
  end
end

