# erfars

A Rust crate that provides safe Rust bindings to the Essential Routines for Fundamental Astronomy ([ERFA](https://github.com/liberfa/erfa)) C library, which is based on the Standards of Fundamental Astronomy ([SOFA](https://www.iausofa.org/index.html)) library published by the International Astronomical Union (IAU).

## To do

`erfars` is currently a work in progress. Most of the [raw C bindings](./src/raw.rs) have safe equivalents but it is a steady effort to implement every single function. Additionally, adequate documentation needs to be added for all the safe functions. The structure of this library also makes it possible for a progressive Rust rewrite, but providing full coverage of the C bindings is the priority.

## Contributing

Contributions are always welcome, and if you have any suggestions or questions you can always reach out to me directly or open an issue.
