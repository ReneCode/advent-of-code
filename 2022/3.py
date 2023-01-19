
import util


def get_data():
  lines = util.read_lines('./3.data')
  return lines

def split_rucksacks(rucksacks):
  all = []
  for rucksack in rucksacks:
    count = len(rucksack)
    if count % 2 > 0:
      raise Exception(f'count of items not even {rucksack}, {count}')
    half = int(count/2)
    first = rucksack[:half]
    second = rucksack[half:]
    all.append((first, second))
  return all


def get_duplicate_item_in_two_parts(two_parts):
  (a,b) = two_parts
  for item in a:
    if item in b:
      return item
  raise Exception(f'not duplicate item found: {a} {b}')

def get_duplicate_item_in_three_parts(three_parts):
  (a,b,c) = three_parts
  for item in a:
    if item in b and item in c:
      return item
  raise Exception(f'not duplicate item found: {a} {b} {c}')




a_z = [chr(n) for n in range(ord('a'), ord('z')+1)]
A_Z = [chr(n) for n in range(ord('A'), ord('Z')+1)]
priority_list = a_z + A_Z

def get_item_priority(item):
  return priority_list.index(item) + 1

rucksacks = get_data()
splitted_rucksacks = split_rucksacks(rucksacks)
total = 0
for two_parts in splitted_rucksacks:
  item = get_duplicate_item_in_two_parts(two_parts)
  prio = get_item_priority(item)
  print(item, prio)
  total += prio

print(f'total priority: {total}')



# split the list in chunks 3 elements each
groups = [rucksacks[k:k + 3] for k in range(0, len(rucksacks), 3)]
total_second = 0
for group in groups:
  item = get_duplicate_item_in_three_parts((group[0], group[1], group[2]))
  prio = get_item_priority(item)
  print(item, prio)
  total_second += prio

print(f'total priority second: {total_second}')
