Implements scoring algorithm for cardiac disease risk under homomorphic encryption (TFHE). The risk factor based algorithm is taken from Carpov and Constantino (2016), and is implemented in the Concrete library for computation on the Torus with Fully Homomorphic Encryption (TFHE) in the Rust programming language.

create_keys

run this to create secret keys for encryption, key switching and bootstrap (may take several hours)
test_basic

test some basic (levelled) operations like addition, multiplication and negation (sign flip)
test_cardio

calculates score for cardiac disease risk according to algortihm under homorphic encryption
References:

S. Carpov, T. H. Nguyen, R. Sirdey, G. Constantino and F. Martinelli, "Practical Privacy-Preserving Medical Diagnosis Using Homomorphic Encryption," https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=7820321
