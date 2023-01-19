
# https://adventofcode.com/2021/day/14

import util

def read_data():
  lines = util.read_data('./14.data')
  start = ''
  rules = {}
  for line in lines:
    if start == '':
      start = line
    elif line != '':
      [match, insert] = line.split(' -> ')
      rules[match] = insert
  return (start, rules)

class Polymer:
  def __repr__(self):
    result = f'{self.name} - {self.generations}/{self.char_counts}'
    return result

  def __init__(self, name):
    self.name = name
    self.generations = [] # string for each generation
    self.char_counts = [] # {} for each generation

  def calc_generations(self, cnt, rules):
    self.generations = []
    for i in range(cnt):
      polymer = self.name
      if i>0:
        polymer = self.generations[i-1]
      polymer = step(polymer, rules)
      self.generations.append(polymer)
      # do not count the right most char
      counts = get_char_counts(polymer)
      self.char_counts.append(counts)

  # abc => { b:1, c:1 }
  def get_right_counts(self, generation):
    poly = self.generations[generation-1]
    key = poly[:1]
    counts = self.char_counts[generation-1]
    result = counts.copy()
    result[key] = result[key] -1
    return result

  # abc => { a:1, b:1  c:1 }
  def get_full_counts(self, generation):
    counts = self.char_counts[generation-1]
    return counts.copy()

def step(polymer, rules):
  result = ''
  poly_len = len(polymer)
  for i in range(poly_len-1):
    pair = polymer[i:i+2]
    result = result + pair[0]
    insert = rules.get(pair)
    if insert != None:
      result = result + insert
  result = result + polymer[-1]
  return result


def get_char_counts(polymer):
  counts = {}
  for w in polymer:
    val = counts.get(w)
    if val == None:
      counts[w] = 1
    else:
      counts[w] = val +1
  return counts


def polymer_to_pairs(polymer):
  l = len(polymer)
  pairs = []
  for i in range(l-1):
    s = polymer[i:i+2]
    pairs.append(s)
  return pairs

def merge_dicts(a, b):
  result = a.copy()
  for key,val in b.items():
    v = result.get(key)
    if v == None:
      v = 0
    result[key] = val + v
  return result

def calc_score(counts):
  min_cnt = None
  max_cnt = None
  for item in counts.items():
    (_, cnt) = item
    if min_cnt == None:
      min_cnt = cnt
    else:
      min_cnt = min(min_cnt, cnt)
    if max_cnt == None:
      max_cnt = cnt
    else:
      max_cnt = max(max_cnt, cnt)
  return max_cnt - min_cnt

def calc_next_polymer(polymer, rules):
  result = ''
  poly_len = len(polymer)
  for i in range(poly_len-1):
    pair = polymer[i:i+2]
    result = result + pair[0]
    insert = rules.get(pair)
    if insert != None:
      result = result + insert
  result = result + polymer[-1]
  return result

(start_polymer, rules) = read_data()

polymers = {}
PRE_CALC_STEPS = 20
print(f'precalc {PRE_CALC_STEPS} steps for the basic rules')
for rule in rules:
  polymer = Polymer(rule)
  polymer.calc_generations(PRE_CALC_STEPS, rules)
  polymers[rule] = polymer


# calc first 20 steps
init_steps = 20
print(f'calc initial {init_steps} steps for the polynom {start_polymer} (like part-1)')
for i in range(init_steps):
  start_polymer = calc_next_polymer(start_polymer, rules)


print(f'calc pairs of a polynom with len {len(start_polymer)}')

steps = 20
pairs = polymer_to_pairs(start_polymer)
result = ''
counts = {}
first = True
for pair in pairs:
  # print(f'calc pair {pair}')
  polymer = polymers[pair]
  part_counts = {}
  if first:
    first = False
    part_counts = polymer.get_full_counts(steps)
  else:
    part_counts = polymer.get_right_counts(steps)
  # print(pair, part_counts)
  counts = merge_dicts(counts, part_counts)

score = calc_score(counts)
print(f'after {steps + init_steps} steps: {counts} Score {score}')


