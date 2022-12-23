from time import time

from encrypt_val import calculate, add, create_sk, get_LWE_str, add_LWE_str

print(calculate)

start = time()
val = calculate(1_000_000_000)
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



for _ in range(10):
    _ = create_sk("00001")

start = time()
_ = create_sk("00001")
end = time()
print(f"elapsed: {end-start}")


for _ in range(10):
    x_LWE = get_LWE_str(0.0, "00001")
    
start = time()
x_LWE = get_LWE_str(1.0, "00001")
end = time()
print(f"type: {type(x_LWE)} calculated in elapsed: {end-start}")
print(f"size of(in kB): {len(x_LWE.encode('utf-8'))/1000}")



y_LWE = get_LWE_str(1.0, "00001")
for _ in range(10):
    z = add_LWE_str(x_LWE, y_LWE, "00001")

start = time()
z = add_LWE_str(x_LWE, y_LWE, "00001")
end = time()
print(f"1 + 1 = {z}, calculated in elapsed: {end-start}")