Trash.exe by Rust
=================

Move file(s) to trash-box of Microsoft Windows.

```
$ trash {OPTION(S)...} {FILENAME(s)...}
```

**Options:**

- `-from-file LISTFILE`: Read filename(s) to remove from the specified file (use `-` to read from standard input) .

It requires VCRUNTIME140.DLL

Install
-------

Download the binary package from [Releases](https://github.com/hymkor/trash-rs/releases) and extract the executable.

### for scoop-installer

```
scoop install https://raw.githubusercontent.com/hymkor/trash-rs/master/trash.json
```

or

```
scoop bucket add hymkor https://github.com/hymkor/scoop-bucket
scoop install trash
```
