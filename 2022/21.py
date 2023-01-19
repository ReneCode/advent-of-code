import util

"""
job:
  yell a specific number
  or
  yell the result of a math operation

  for math operation needed two other monkeys
"""


HUMAN_NAME = "humn"
ROOT_NAME = "root"

class Monkey:
  def __init__(self, name, nr, ops):
    self.name = name
    self.nr = nr 
    if nr == None:
      self.result = None
      self.first_name = ops[0]
      self.operation = ops[1]
      self.second_name = ops[2]
    else:
      self.result = nr

  def __repr__(self):
    out = f"{self.name}:"
    if self.nr != None:
      return f"{out} {self.nr}"
    else:
      return f"{out} {self.first_name} {self.operation} {self.second_name}"


def get_data():
  lines = util.read_lines('./21.data')
  monkeys = {}
  for line in lines:
    tok = line.split(":")
    name = tok[0]
    tok = tok[1].strip().split((" "))
    nr = None
    if tok[0].isdecimal():
      nr = int(tok[0])
      monkey = Monkey(name, nr, None)
    else:
      first_name = tok[0]
      operation = tok[1]
      second_name = tok[2]
      monkey = Monkey(name, None, (first_name, operation, second_name))

    monkeys[name] = monkey
  return monkeys



def solve_1(monkeys, name):
  monkey = monkeys[name]

  if monkey.nr != None:
    return monkey.nr

  first_result = solve_1(monkeys, monkey.first_name)
  second_result = solve_1(monkeys, monkey.second_name)
  result = None
  if monkey.operation == "+":
    result = first_result + second_result
  elif monkey.operation == "-":
    result = first_result - second_result
  elif monkey.operation == "/":
    result = int(first_result / second_result)
  elif monkey.operation == "*":
    result = first_result * second_result
  else:
    raise Exception(f"ups, wrong operation {monkey.operation}")
  return result


def fill_result(monkeys, name):
  if name == HUMAN_NAME:
    return None

  monkey = monkeys[name]

  if monkey.nr != None:
    return monkey.nr

  first_result = fill_result(monkeys, monkey.first_name)
  second_result = fill_result(monkeys, monkey.second_name)
  if first_result == None or second_result == None:
    return None

  result = None
  if monkey.operation == "+":
    result = first_result + second_result
  elif monkey.operation == "-":
    result = first_result - second_result
  elif monkey.operation == "/":
    result = int(first_result / second_result)
  elif monkey.operation == "*":
    result = first_result * second_result
  else:
    raise Exception(f"ups, wrong operation {monkey.operation}")
  monkey.result = result
  return result


def find_human(monkeys, name, way):
  if name == HUMAN_NAME:
    way.append(name)
    return True

  monkey = monkeys[name]
  if monkey.nr != None:
    return False
  
  if find_human(monkeys, monkey.first_name, way):
    way.append(name)
    return True
  if find_human(monkeys, monkey.second_name, way):
    way.append(name)
    return True
  return False



monkeys = get_data()
# print (monkeys)

# root = monkeys["ptdq"]
# root.hello()


# result_1 = solve_1(monkeys, ROOT_NAME)
# print(f'part-1: {result_1}')


humn_monkey = monkeys[HUMAN_NAME]
humn_monkey.result = None

way = []
find_human(monkeys, ROOT_NAME, way)
print(way)

fill_result(monkeys, ROOT_NAME)




wanted_result = None
for name in reversed(way):
  if name == HUMAN_NAME:
    break
  monkey = monkeys[name]
  first_monkey = monkeys[monkey.first_name]
  second_monkey = monkeys[monkey.second_name]
  if monkey.name == ROOT_NAME:
    if first_monkey.result != None:
      wanted_result = first_monkey.result
    else:
      wanted_result = second_monkey.result
  else:
    if wanted_result == None:
      raise Exception(f'ups result missing for {monkey.name}')
    if monkey.operation == "+":
      if first_monkey.result != None:
        wanted_result = wanted_result - first_monkey.result
      else:
        wanted_result = wanted_result - second_monkey.result
    elif monkey.operation == "-":
      if first_monkey.result != None:
        wanted_result = first_monkey.result - wanted_result
      else:
        wanted_result = second_monkey.result + wanted_result
    elif monkey.operation == "*":
      if first_monkey.result != None:
        wanted_result = int(wanted_result / first_monkey.result)
      else:
        wanted_result = int(wanted_result / second_monkey.result)
    elif monkey.operation == "/":
      if first_monkey.result != None:
        wanted_result = first_monkey.result / wanted_result
      else:
        wanted_result = second_monkey.result * wanted_result
    else:
      raise Exception(f"bad operator {monkey.operation}")
  print(f'{monkey.name} calulated: {wanted_result}')

print(f'part-2: {wanted_result}')









# for i in range(10000):
#   monkeys[YOU_NAME].nr = i
#   result_2 = solve_2(monkeys, ROOT_NAME)
#   if result_2:
#     print(f"part-2 gefunden: {i}")
#     break
# print("sorry, not found")