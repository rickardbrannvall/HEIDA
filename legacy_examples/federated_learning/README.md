# FHEFL - FHE enchanced federated learning

This repository contains the Python and Rust code used for the numerical investigations.  

## CKKS operations

Benchmarking of CKKS operations happens in Python notebooks using the TenSEAL library.

## TFHE operations

For TFHE the noteboks subprocess Rust executables coded using the Concrete library. 

Rust source code in src/bin directory.

The code is compiled by

cargo build —release

Examples of command line use:

$ target/release/create_keys 2040
$ target/release/add_vectors keys/def_80_512_1 6 4 0.0 1.0 4 1 0.5 0.3 0.7 0.4
$ target/release/enc_vector keys/def_80_512_1 temp/test.enc 6 4 0.0 1.0 4 0.5 0.3 0.7 0.4
