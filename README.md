<div align="center">

# Verified Toy Fibonacci CLI

**Verified, toy CLI for calculating the n-th fibonacci number.**

</div>

This is a toy project experimenting with dafny and the translation to rust.
The fibonacci algorithm is verified in Dafny (which of course isn't really a hard thing to do).
The Dafny code is in [`src/main.dfy`](./src/main.dfy).
The verified and to rust translated code is also checked out in git under the [`translation/main-rust`](./translation/main-rust/) directory.

## Compilation

To compile the in the VCS checked out Rust code, run:

```
cargo build --release
```

The compiled executable is in `target/release`.


## Compilation from Dafny

To verify, translate and compile the Dafny code, install [task](https://taskfile.dev/) and then run:

```
task brs
```

The compiled executable is in `target/release`.

## Verification of Dafny Code

To just verify the Dafny code, install [task](https://taskfile.dev/) and then run:

```
task v
```



