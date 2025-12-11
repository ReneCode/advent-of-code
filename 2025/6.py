
import math

def get_tokens(line, sep=" "):
  return list(filter(lambda x: x != "", line.split(sep)))


def part1():

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



def part2():

  with open("6.data") as f:
    lines = f.readlines()
    lines = [line.strip("\n") for line in lines]

  max_len = max([len(line) for line in lines])
  for i in range(len(lines)):
    lines[i] = lines[i].ljust(max_len, " ")
  width = len(lines[0])

  total_sum = 0
  numbers = []
  for idx in reversed(range(width)):
    col_chars = [lines[row_idx][idx] for row_idx in range(len(lines))]
    nr_str = "".join(col_chars[:-1]).strip()
    if nr_str == "":
      numbers = []
      continue
    nr= int(nr_str)
    numbers.append(nr)
    op = col_chars[-1]
    if op == " ":
      continue

    result = 0
    if op == "+":
      result = sum(numbers)
    elif op == "*":
      result = math.prod(numbers)
    print ( "nr:", numbers, "op:", op, "result:", result)
    total_sum += result

  print("part2 total sum:", total_sum)




# part1()
part2()