from collections import namedtuple

import util

Item = namedtuple('Item', ['idx', 'val'])

def get_data():
  lines = util.read_lines("./20.data")
  numbers = [int(n) for n in lines]
  zeoros = [n for n in numbers if n == 0]
  if len(zeoros) != 1:
    raise Exception(f"ups 0 found more than once")
  result = []
  for idx in range(len(numbers)):
    result.append( Item(idx, numbers[idx]) )
  return result


# [a,b,c,d]   move b +2 steps
# [a,c,b,d]   1.step
# [b,a,c,d]   2.step wrap around
def move_item(sequence, item):
  (idx, val) = item
  if val == 0:
    return sequence

  direction = util.signum(val)
  sequence_len = len(sequence)

  start_idx = sequence.index(item)
  if start_idx < 0:
    raise Exception(f"item not found {item}")

  if sequence[start_idx] != item:
    raise Exception(f"ups got not the right item {sequence[start_idx]} / {item}")

  result = sequence.copy()

  end_idx = (start_idx + val + sequence_len - 1) % (sequence_len-1)
  result.remove(item)
  result.insert(end_idx, item)
  return result


def print_sequence(sequence):
  result = [t.val for t in sequence]
  print(result)


def calc_coord(sequence, nth):
  sequence_len = len(sequence)
  for idx in range(sequence_len):
    if sequence[idx].val == 0:
      break
  print(f"{idx}")
  idx += nth
  idx = idx % sequence_len
  result = sequence[idx].val
  print(f"{idx} = {result}")
  return result



sequence = get_data()
original_sequence = sequence.copy()
# print_sequence(sequence)
sequence_len = len(sequence)
for idx in range(sequence_len):
  item = original_sequence[idx]
  sequence = move_item(sequence, item)
  # print_sequence(sequence)

# print_sequence(sequence)

first = calc_coord(sequence, 1000)
second = calc_coord(sequence, 2000)
third = calc_coord(sequence, 3000)
total = first + second + third
print(f'part-1 total result: {total}')

# part-2

decryption_key = 811589153
sequence = get_data()
sequence = [Item(i.idx, i.val*decryption_key) for i in sequence]
original_sequence = sequence.copy()
sequence_len = len(sequence)
for n in range(10):
  for idx in range(sequence_len):
    item = original_sequence[idx]
    sequence = move_item(sequence, item)

  # print_sequence(sequence)

first = calc_coord(sequence, 1000)
second = calc_coord(sequence, 2000)
third = calc_coord(sequence, 3000)
total = first + second + third
print(f'part-2 total result: {total}')


