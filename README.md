# uenc/udec

Simple command line url encoder/decoder.


## Installation

With rust and cargo already installed:

```shell
$ cargo install uenc
```

## Usage

```shell
$ uenc tag/global.must
=> tag%2Fglobal.must

$ udec `uenc tag/global.must`
=> tag/global.must
```

To be interactive, use `-n` options that keeps newline:

```shell
$ uenc -n
tag/global.must
=> tag%2Fglobal.must
```
