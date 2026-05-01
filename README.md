# rudof-fhir

This repo contains some examples and schemas related with the **FHIR ShEx schema** and how we can handle it with [`rudof`](https://github.com/rudof-project/rudof).


## Compiling and running

The code is a `main` function that wraps the rudof validator. It can be compiled with [cargo](https://doc.rust-lang.org/cargo/).

For more performance, we recommend to compile it using:

```sh
cargo build --release
```

and to use the binary located in:

```sh
target/release/rudof_fhir
```