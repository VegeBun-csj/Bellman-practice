# Bellman-practice
This is a zkp practice by the bellman library

This parctice will follow these steps:
* generate proof and circuit by the snarkjs and circom compiler with `bls12_381` curve
* using the bellman library to verify the above proof
    - convert the `json` type to `Affine` type in rust
    - convert the `json` type to `prepared` type
    - integrate them to verify
