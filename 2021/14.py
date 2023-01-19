
# https://adventofcode.com/2021/day/14

import util

def read_data():
  lines = util.read_data('./14-example.data')
  start = ''
  rules = {}
  for line in lines:
    if start == '':
      start = line
    elif line != '':
      [match, insert] = line.split(' -> ')
      rules[match] = insert
  return (start, rules)


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

def calc_score(polymer):
  stats = {}
  for w in polymer:
    val = stats.get(w)
    if val == None:
      stats[w] = 1
    else:
      stats[w] = val +1
  print(stats)
  min_cnt = None
  max_cnt = None
  for item in stats.items():
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

    

(polymer, rules) = read_data()
print(polymer)
for i in range(20):
  polymer = step(polymer, rules)
  score = calc_score(polymer)
  print(f'After step {i+1}: {polymer[:20]}  {score}')
