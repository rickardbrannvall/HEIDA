# HEIDA

This repository holds Python code / Jupyter Notebooks with examples on application of FHE to problems in health data science. Note that these are stand-alone examples used to illustrate the concepts. Therefore public and private keys are not handled separately - indeed encyrption, homomorphic data processing, and decryption of results happen in the same notebook on the same server. In real applications this would be split into separate programs run on different servers. We hope that examples in this repo can be modified for simple numerical investigations and benchmarking of your own algorithms.

Benchmarking of operations happens in Python notebooks using the TFHE Concrete and TenSEAL CKKS libraries.

1. assumes code is compiled and run in docker container: jupyter/datascience-notebook (or similar)
2. install all libraries in requirements.txt

Alternatively you can setup manually in ubuntu with at least jupyterlab installed (although there may be further requirements).

Directories:
- legacy_examples -- these are examples from a previous iteration of this repo built on TFHE Concrete in Rust
- python_examples -- these are the more current examples only in Python for both TenSEal CKKS and TFHE Concrete
