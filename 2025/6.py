
import math

def get_tokens(line, sep=" "):
  return list(filter(lambda x: x != "", line.split(sep)))


with open("6.data") as f:
  lines = f.readlines()
  lines = [line.strip("\n") for line in lines]


last_line = lines[-1]
lines = lines[:-1]
operations = get_tokens(last_line, " ")

numbers = []
for line in lines:
  parts = get_tokens(line, " ")
  numbers.append([int(p) for p in parts])


total_sum = 0
for i in range(len(operations)):
  op = operations[i]
  nrs = [row[i] for row in numbers]
  if op == "+":
    result = sum(nrs)
  elif op == "*":
    result = math.prod(nrs)
  else:
    raise Exception("unknown operation:", op)
  total_sum += result

print("total sum:", total_sum)  