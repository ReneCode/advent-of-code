import util
from functools import cmp_to_key

STATE_START = 1
STATE_READ_ITEMS = 2

def get_items(line):
  items = []
  item = ""
  state = STATE_START
  inner_array = 0
  array = 0
  for c in line:
    if c == '[':
      if state == STATE_START:
        inner_array = 0
        item = ""
        state = STATE_READ_ITEMS
      else:
        item += c
        inner_array += 1
    elif c == ']':
      if inner_array > 0:
        inner_array -= 1
        item += c
      else:
        if item != "":
          items.append(item)
        item = ""
    elif c == ',':
      if inner_array == 0:
        if item != "":
          items.append(item)
        item = ""
      else:
        item += c
    else:
      item += c
  return items


def test_get_items_a():
  items = get_items('[1,2,3]')
  assert items == ['1','2','3']

def test_get_items_b():
  items = get_items('[[1,2],3]')
  assert items == ['[1,2]','3']

def test_get_items_c():
  items = get_items('[[1],[2,3,4]]')
  assert items == ['[1]','[2,3,4]']

def test_get_items_d():
  items = get_items('[]')
  assert items == []

ORDER_OK = 1
ORDER_BAD = 2
ORDER_CONTINUE = 3

def order_correct(left, right):
  l_isdec = left.isdecimal()
  r_isdec = right.isdecimal()
  if l_isdec and r_isdec:
    l_val = int(left)
    r_val = int(right)
    if l_val < r_val:
      return ORDER_OK
    elif l_val > r_val:
      return ORDER_BAD
    else:
      return ORDER_CONTINUE
  elif l_isdec and not r_isdec:
    return order_correct("[" + left + "]", right)      
  elif not l_isdec and r_isdec:
    return order_correct(left, "[" + right + "]")
  elif not l_isdec and not r_isdec:
    # both are lists
    left_items = get_items(left)
    right_items = get_items(right)
    left_len = len(left_items)
    right_len = len(right_items)
    for i in range( min(left_len, right_len)):
      l = left_items[i]
      r = right_items[i]
      order = order_correct(l, r)
      if order != ORDER_CONTINUE:
        return order
        
    if left_len < right_len:
      return ORDER_OK
    elif left_len > right_len:
      return ORDER_BAD
    else:
      return ORDER_CONTINUE

  else:
    raise Exception(f'bad situation')

def test_order_correct_1():
  left = "[1,1,3,1,1]"
  right = "[1,1,5,1,1]"
  correct = order_correct(left, right)
  assert correct == ORDER_OK

def test_order_correct_3():
  left = "[[1],[2,3,4]]"
  right = "[[1],4]"
  correct = order_correct(left, right)
  assert correct == ORDER_OK

def test_order_correct_3():
  left = "[9]"
  right = "[[8,7,6]]"
  correct = order_correct(left, right)
  assert correct == ORDER_BAD

def test_order_correct_4():
  left = "[[4,4],4,4]"
  right = "[[4,4],4,4,4]"
  correct = order_correct(left, right)
  assert correct == ORDER_OK

def test_order_correct_5():
  left = "[7,7,7,7]"
  right = "[7,7,7]"
  correct = order_correct(left, right)
  assert correct == ORDER_BAD

def test_order_correct_6():
  left = "[]"
  right = "[3]"
  correct = order_correct(left, right)
  assert correct == ORDER_OK

def test_order_correct_7():
  left = "[[[]]]"
  right = "[[]]"
  correct = order_correct(left, right)
  assert correct == ORDER_BAD

def test_order_correct_8():
  left = "[1,[2,[3,[4,[5,6,7]]]],8,9]"
  right = "[1,[2,[3,[4,[5,6,0]]]],8,9]"
  correct = order_correct(left, right)
  assert correct == ORDER_BAD

def test_order_correct_test():

  left = "[[[1,6,[1,9,0,9],6]]]"
  right = "[[],[[[5,6,3],6,[6,5,3,3]],8,3],[],[4]]"
  correct = order_correct(left, right)
  assert correct == ORDER_BAD


class Pair:
  def __init__(self, left, right):
    self.left = left
    self.right = right

  def __repr__(self):
    return f'\n{self.left}\n{self.right}\n'

  def order_correct(self):
    left_items = get_items(self.left)
    right_items = get_items(self.right)
    return order_correct(self.left, self.right) == ORDER_OK


def get_data():
  lines = util.read_lines('./13.data')
  first_line = None
  pairs = []
  for line in lines:
    if line == "":
      continue
    if first_line == None:
      first_line = line
    else:
      pair = Pair(first_line, line)
      pairs.append(pair)
      first_line = None
  return pairs


# test_order_correct_1()

pairs = get_data()
idx = 0
total = 0
for pair in pairs:
  idx += 1
  if pair.order_correct():
    # print(f'pair with index:{idx} is ok')
    total += idx
print(f'part-1: total ok pairs: {total}')

all = []
for pair in pairs:
  all.append(pair.left)
  all.append(pair.right)

divider_packages = [
"[[2]]",
"[[6]]"
]
all.extend(divider_packages)

def compare(a, b):
  order = order_correct(a, b)
  if order == ORDER_OK:
    return -1
  else:
    return 1

all.sort(key=cmp_to_key(compare))
idx_1 = 1+ all.index(divider_packages[0])
idx_2 = 1+ all.index(divider_packages[1])
print(f'part-2 decoder key: {idx_1 * idx_2}')
