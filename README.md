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
lgplv2.1
lgplv3
postgresql
```

Feel free to contribute and add more--just add a file in `/templates` and a string in `src/main.rs`.

