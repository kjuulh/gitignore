# Git ignore (kignore)

Is an extension for easily adding ignored files to `.gitignore files`, when
added it will by default also try to remove any files matching the pattern added
to .gitignore, this is by default also run in interactive mode, giving you the
option to confirm or deny

```bash
$ kignore 'node_modules/'
found .gitignore .../some-path/.gitignore
Added node_modules/ to .gitignore
Removed node_modules/ from git index
```

```bash
$ git ignore 'node_modules/'
found .gitignore .../some-path/.gitignore
Added node_modules/ to .gitignore
Removed node_modules/ from git index
```

## Installation

### Cargo

Cargo will only pull the `kignore` command and won't add a subcommand to `git.

```bash
$ cargo install kignore
$ cargo binstall kignore # binstall will warn that git.front.kjuulh.io isn't a valid repo, it is still installable though

```

#### Post install

To get the `git ignore` subcommand working you will need to have the file
git-ignore available on your path

```
# zsh
eval "kignore init zsh"

# shell
eval "kignore init sh"

# bash
eval "kignore init bash"
```
