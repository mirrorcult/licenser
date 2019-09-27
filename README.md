# licenser

Basic CLI utility that adds a LICENSE file in your working directory given a certain template name

## Usage

```bash
licenser license-name [file_name]
```

Default file name is `LICENSE`.

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

