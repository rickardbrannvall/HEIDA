import numpy as np
from time import time
from target.release.libheida import (create_sk, 
    get_LWE_str, add_LWE_str, decrypt_LWE_str,
    get_VecLWE_str, add_VecLWE_str, decrypt_VecLWE_str,
    get_list_VecLWE_str)


print("\nCreating Secret key!")
for _ in range(10):
    _ = create_sk(1024, -40, "00001")

start = time()
_ = create_sk(1024, -40, "00001")
end = time()

print(f"elapsed: {end-start}")



print("\nEncrypt and move to string!")
for _ in range(10):
    x_LWE = get_LWE_str(0.0, "00001")

start = time()
x_LWE = get_LWE_str(1.0, "00001")
end = time()

print(f"type: {type(x_LWE)}, Encrypted in elapsed: {end-start}")
print(f"size of(in kB): {len(x_LWE.encode('utf-8'))/1000}")



print("\nString to encrypted value and then decrypt")
for _ in range(10):
    _ = decrypt_LWE_str(x_LWE, "00001")

start = time()
_ = decrypt_LWE_str(x_LWE, "00001")
end = time()

print(f"Decrypting elapsed: {end-start}")



print("\n2x strings to encryted values and then add")
y_LWE = get_LWE_str(1.0, "00001")
for _ in range(10):
    _ = add_LWE_str(x_LWE, y_LWE, "00001")

start = time()
z_LWE = add_LWE_str(x_LWE, y_LWE, "00001")
end = time()

z = decrypt_LWE_str(z_LWE, "00001")
print(f"1 + 1 = {z}, calculated in elapsed: {end-start}")



## ---- VECTORS ---- ##
my_list = [1.0 for _ in range(14_000)]

print("\nEncrypt and move to string!")
for _ in range(10):
    x_VecLWE = get_VecLWE_str(my_list, "00001")

start = time()
x_VecLWE = get_VecLWE_str(my_list, "00001")
end = time()

print(f"type: {type(x_VecLWE)}, Encrypted in elapsed: {end-start}")
print(f"size of(in kB): {len(x_VecLWE.encode('utf-8'))/1000}")



print("\nString to encrypted value and then decrypt")
for _ in range(10):
    _ = decrypt_VecLWE_str(x_VecLWE, "00001")

start = time()
_ = decrypt_VecLWE_str(x_VecLWE, "00001")
end = time()

print(f"Decrypting elapsed: {end-start}")


print("\n2x strings to encryted values and then add")
y_VecLWE = get_VecLWE_str(my_list, "00001")
for _ in range(10):
    z_VecLWE = add_VecLWE_str(x_VecLWE, y_VecLWE, "00001")

start = time()
z_VecLWE = add_VecLWE_str(x_VecLWE, y_VecLWE, "00001")
end = time()

z = decrypt_VecLWE_str(z_VecLWE, "00001")
print(f"z = {np.mean(z)} +- {np.std(z)}, calculated in elapsed: {end-start}")



## ---- LISTS ---- ##
# my_list = [1.0 for _ in range(100_000)]

print("\nEncrypt and move to string!")
for _ in range(10):
    x_VecLWE = get_list_VecLWE_str(my_list, "00001")

start = time()
x_VecLWE = get_list_VecLWE_str(my_list, "00001")
end = time()

print(f"type: {type(x_VecLWE)}, Encrypted in elapsed: {end-start}")
# print(f"size of(in kB): {len(x_VecLWE.encode('utf-8'))/1000}")