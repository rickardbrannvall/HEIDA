# FHEFL - FHE enchanced federated learning

This repository holds Python code / Jupyter Notebooks for numerical investigations and benchmarking.

Note that these are stand-alone examples used to illustrate how FHE can enhance federated learning.

It is a single server simulation of federated learning that does not distribute data over multiple nodes.

Benchmarking of operations happens in Python notebooks using the TenSEAL CKKS library.

For these examples one can use the docker container openmined/tenseal with Tenseal preinstalled.

Files:
- nb0_requirements.ipynb installs some of the required libraries for the two examples in this directory
- nb1_logistic_regression.ipynb provides an example for the logistic regression model on tabular data
- nb2_resenet_medmnist.ipynb provides an exmaple for training a ResNet18 model on medical images