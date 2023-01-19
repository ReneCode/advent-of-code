
# https://adventofcode.com/2021/day/7

import util

def get_start_positions():
  input_data = util.read_data('./7.data')
  line = input_data[0]
  input_positions = [int(n) for n in line.split(",")]
  max_pos = 0
  positions = []
  for pos in input_positions:
    max_pos = max(max_pos, pos)
  positions = [0] * (max_pos+1)
  for pos in input_positions:
    positions[pos] = positions[pos] +1
  return positions

cache_costs = {}

def calc_costs(delta):
  cost = cache_costs.get(delta)
  if cost != None:
    return cost

  cost = 0
  for i in range(delta):
    cost = cost + (i+1)
  cache_costs[delta] = cost
  return cost


def calc_moving_costs(positions, target_pos):
  total = 0
  for i, count in enumerate(positions):
    # part 1
    # single_cost = abs(i - target_pos)

    # part 2
    delta = abs(i - target_pos)
    single_cost = calc_costs(delta)  

    total = total + count * single_cost
  return total

# position[index] = count of crabs on that index position
positions = get_start_positions()
min_cost = -1
cnt = len(positions)
for pos in range(cnt):
  cost = calc_moving_costs(positions, pos)
  print(f'{pos}/{cnt} {cost}')
  if min_cost < 0:
    min_cost = cost
  else:
    min_cost = min(min_cost, cost)

# print(positions)
print(min_cost)