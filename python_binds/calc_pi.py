from time import time

from target.release.libheida import calculate, add

import numpy as np

print(calculate)

start = time()
val = calculate(100_000_000)
end = time()

print(f"value: {val} calculated in elapsed: {end-start}")


x, y = 2_000, 3_000
for _ in range(10):
    val = x + y
start = time()
val = x + y
end = time()

print(f"value: {val} calculated in elapsed: {end-start}")



for _ in range(10):
    val = add(x, y)

start = time()
val = add(x, y)
end = time()

print(f"value: {val} calculated in elapsed: {end-start}")