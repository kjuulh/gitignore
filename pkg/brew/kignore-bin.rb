class KignoreBin < Formula
  desc "Easily add items to .gitignore and cleanup afterwards"
  homepage "https://github.com/kjuulh/gitignore"
  version "0.1.2"
  license "MIT"

  if OS.mac?
    url "https://github.com/kjuulh/kignore/releases/download/#{version}/kignore-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "8eacb5cf0d4116161f291006bc6c91b8ce2760eab0fcf8cb79a62c246bdfff89"
  end

  def install
    bin.install "kignore"
  end
end

