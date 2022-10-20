# Git ignore

Is an extension for easily adding ignored files to `.gitignore files`, when
added it will by default also try to remove any files matching the pattern added
to .gitignore, this is by default also run in interactive mode, giving you the
option to confirm or deny

```bash
$ git ignore 'node_modules/'
Added node_modules/ to .gitignore
Searching env for pattern...

found:
<gitroot>/client/node_modules
? Remove from git state? (Y)es/(N)o/(C)ontinue/(A)bort
```
