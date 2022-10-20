class Kignore < Formula
  desc "Easily add items to .gitignore and cleanup afterwards"
  homepage "https://github.com/kjuulh/gitignore"
  version '0.1.1'
  license "MIT"

  if OS.mac?
    url "https://github.com/kjuulh/kignore/releases/download/#{version}/kignore-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 ""
  end

  def install
    bin.install "kignore"
  end
end

