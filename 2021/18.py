
# https://adventofcode.com/2021/day/18

import util

def is_int(obj):
  return isinstance(obj, int)

def is_decimal(string):
  return string.isdecimal()

def is_pair(obj):
  return isinstance(obj, Pair)

def pair_to_array(pair):
  items = []
  for w in pair:
    if w == '[' or w == ']':
      items.append(w)
    elif w == ',':
      items.append(w)
    else:
      items.append(int(w))
  return items

def array_to_pair(items):
  result = ''
  last_item = None
  for item in items:
    if item == '[' or item == ']' or item == ',':
      result = result + item
    else:
      result = result + str(item)
  return result
# 123[4,5]67  =>  123067
def get_and_clear_pair(pair, idx):
  left = None
  right = None
  item = pair.pop(idx)
  assert(item == '[')
  left = pair.pop(idx)
  item = pair.pop(idx)
  right = pair.pop(idx)
  pair[idx] = 0
  return (left, right)


def add_next_int(result, val, idx, delta_idx):
  i = idx + delta_idx
  while i>= 0 and i < len(result):
    if is_int(result[i]):
      result[i] = result[i] + val
      return
    i = i + delta_idx

def explode(items):
  count_open_brackets = 0
  ignore_until_bracket_close = False
  for idx, item in enumerate(items):
    if item == '[':
      count_open_brackets = count_open_brackets +1
      if count_open_brackets > 4:
        (left, right) = get_and_clear_pair(items, idx)
        add_next_int(items, left, idx, -1)
        add_next_int(items, right, idx, +1)
        return True
    elif item == ']':
      count_open_brackets = count_open_brackets -1
  return False

def insert_pair(items, idx, left, right):
  items.pop(idx)
  items.insert(idx, '[')
  items.insert(idx+1, left)
  items.insert(idx+2, ',')
  items.insert(idx+3, right)
  items.insert(idx+4, ']')


def split(items):
  for idx, item in enumerate(items):
    if is_int(item):
      if item >= 10:
        left = right = item // 2
        if item % 2 > 0:
          right = right + 1
        insert_pair(items, idx, left, right)
        return True
  return False


def reduce(pair):
  items = pair_to_array(pair)
  cont = True
  while cont:
    while explode(items):
      pass
    cont = split(items)

  pair = array_to_pair(items)
  return pair

def add(p1, p2):
  pair = f'[{p1},{p2}]'

  pair = reduce(pair)
  return pair

def add_all(pairs):
  result = pairs[0]
  for i in range(1, len(pairs)):
    result = add(result, pairs[i])
  return result


class Pair:
  def __init__(self, left, right):
    self.left = left
    self.right = right

  def magnitude(self):
    result = 0
    if is_pair(self.left):
      result = result + 3 * self.left.magnitude()
    else:
      result = result + 3 * self.left
    if is_pair(self.right):
      result = result + 2 * self.right.magnitude()
    else:
      result = result + 2 * self.right
    return result

def parse_pair(tokens):
  tok = tokens.pop(0)
  assert(tok == '[')
  if is_decimal(tokens[0]):
    left = int(tokens.pop(0))
  else:
    left = parse_pair(tokens)
  komma = tokens.pop(0)
  if is_decimal(tokens[0]):
    right = int(tokens.pop(0))
  else:
    right = parse_pair(tokens)
  tok = tokens.pop(0)
  assert(tok == ']')
  return Pair(left, right)


def calc_magintude(pair):
  tokens = [w for w in pair]
  tree = parse_pair(tokens)
  return tree.magnitude()

def part_1():
  lines = util.read_data('./18-example.data')
  result = add_all(lines)
  print(result)
  print(calc_magintude(result))

def part_2():
  lines = util.read_data('./18.data')
  max_magnitude = 0
  for i in range(len(lines)):
    for j in range(len(lines)):
      if i != j:
        result = add(lines[i], lines[j])
        magnitude = calc_magintude(result)
        max_magnitude = max(max_magnitude, magnitude)
        print(f'{i} {j} {magnitude}')
  print(f'max magnitude: {max_magnitude}')

part_2()