<div align="center">

# erfars

</div>

This repository contains the **erfars** Rust crate, which provides safe Rust bindings to the Essential Routines for Fundamental Astronomy ([ERFA](https://github.com/liberfa/erfa)) C library, which is based on the Standards of Fundamental Astronomy ([SOFA](https://www.iausofa.org/index.html)) library published by the International Astronomical Union (IAU).

Documentation is currently not available, but the function interfaces are very similar to the C interfaces so please consult the original ERFA docs for now.

The only notable difference between this library and ERFA is that warning codes returned by ERFA are ignored by the safe Rust functions. This isn't a permanent design decision, I just haven't figured out an elegant way of returning warnings without forcing the user to pass mutable references everywhere. If you absolutely want the warning information, you can use the [raw bindings](./src/raw.rs) directly.

## Contributing

Contributions are always welcome in the form of Github issues or pull requests, and if you have any suggestions or questions you can always reach out to me directly.

## Licensing

This library is licensed under either the MIT License or the Apache 2.0 License at your option. A copy of the original ERFA license is provided [here](./LICENSE-ERFA.txt).
