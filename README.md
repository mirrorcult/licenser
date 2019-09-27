# licenser

Basic CLI utility that adds a LICENSE file in your working directory given a certain template name

## Usage

> licenser license-name [license-file]

```bash
$ licenser mit LICENSE-MIT
$ licenser apache LICENSE # defaults to LICENSE

# LICENSE-MIT:
    > Permission is hereby granted, free of charge...

# LICENSE
    > TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION...
```

## Valid License Names

```
agpl
gplv2
gplv3
mit
apache
wtfpl
afl
artistic
bsd2
bsd3
bsl
ccbysa
ccby
lgplv21 (lgplv21 and not v2.1 because github thinks a .1 file is a "roff" file for some reason)
lgplv3
postgresql
```

Feel free to contribute and add more--just add a file in `/templates` and a string in `src/main.rs`.

