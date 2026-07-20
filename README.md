<div align="center">

# Toy Verified Fibonacci CLI

**Verified CLI for calculating the n-th fibonacci number.**

</div>

This is a toy project experimenting with dafny and the translation to rust.
The fibonacci algorithm is verified in Dafny (which of course isn't really a hard thing to do).
The verified and to rust translated code is also checked out in git under the `translation/main-rust` directory.

## Compilation

```
cargo build --release
```

The compiled executable is in `target/release`.


## Compilation from Dafny

Install [task](https://taskfile.dev/), then run:

```
task brs
```

The compiled executable is in `target/release`.

## Verification of Dafny Code

Install [task](https://taskfile.dev/), then run:

```
task v
```



