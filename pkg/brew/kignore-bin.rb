class KignoreBin < Formula
  desc "Easily add items to .gitignore and cleanup afterwards"
  homepage "https://github.com/kjuulh/gitignore"
  version "0.1.3"
  license "MIT"

  if OS.mac?
    url "https://github.com/kjuulh/gitignore/releases/download/#{version}/kignore-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "fa4e520854f0cc8222625b0398c778d4f474dd7a9ad1da1dd9a326ff7893bd44"
  end

  def install
    bin.install "kignore"
    bin.install "git-ignore"
    bin.install "git-ignore" => "git-kignore"
  end
end

