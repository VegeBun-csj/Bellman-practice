# Bellman-practice
This is a zkp practice by the bellman library

This parctice will follow these steps:
* generate proof and circuit by the snarkjs and circom compiler with `bls12_381` curve
* using the bellman library to verify the above proof
    - snarkjs: convert the `String（big int)` type to `uncompressed array` type 
    - bellman: convert the `uncompressed array` type to `Affine` type
