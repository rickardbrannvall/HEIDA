from time import time

from target.release.libheida import calculate, add

import numpy as np

print(calculate)

start = time()
val = calculate(100_000_000)
end = time()

print(f"value: {val} calculated in elapsed: {end-start}")



for _ in range(10):
    val = 2000 + 3000
start = time()
val = 2000 + 3000
end = time()

print(f"value: {val} calculated in elapsed: {end-start}")



for _ in range(10):
    val = add(2000, 3000)

start = time()
val = add(2000, 3000)
end = time()

print(f"value: {val} calculated in elapsed: {end-start}")



