import util
"""
ore -> clay-collector
       clay -> obsidian-collector
               obsidian -> geode-collector
                           geode

at start you have one ore-collector

quality level of blueprint = id of blueprint * count of geodes

total = sum of all quality levels
"""

ORE = 0
CLAY = 1
OBSIDIAN = 2
GEODE = 3

PRODUCTS = [ORE, CLAY, OBSIDIAN, GEODE]


class Blueprint:
  def __init__(self, costs):
    self.costs = costs


def get_data():
  lines = util.read_lines("./19.data")
  blueprints = []
  for line in lines:
    tok = line.split(" ")
    # ore
    ore_robot_cost = [int(tok[6]), 0, 0, 0]
    # ore
    clay_robot_cost = [int(tok[12]), 0, 0, 0]
    # ore, clay
    obsidian_robot_cost = [int(tok[18]), int(tok[21]), 0, 0]
    # ore, obsidian
    geode_robot_cost = [int(tok[27]), 0, int(tok[30]), 0]
    costs = [
        ore_robot_cost,
        clay_robot_cost,
        obsidian_robot_cost,
        geode_robot_cost
    ]
    blueprints.append(Blueprint(costs))
  return blueprints


def can_build_robot(cost, inventory):
  for p in PRODUCTS:
    if inventory[p] < cost[p]:
      return False
  return True

def add_values(a, b):
  # return a + b
  result = a.copy()
  for i in PRODUCTS:
    result[i] += b[i]
  return result

def subtract_values(a, b):
  # return a - b
  result = a.copy()
  for i in PRODUCTS:
    result[i] -= b[i]
  return result

def compare_factory(a):
  # GEODE is the most valuable product, next OBSIDIAN, ...a
  # weight - factor 1000, 100, 10, 1
  def val_mined(factory):
    mined = factory[3]
    return mined[ORE] + mined[CLAY]*10 + mined[OBSIDIAN]*100 + mined[GEODE]*1000

  va = val_mined(a)
  return va

def get_max_geode(blueprint, max_time, max_que_size):
  init_robots = [1,0,0,0]
  init_inventory = [0,0,0,0]
  init_mined = [0,0,0,0]
  first_factory = (0, init_robots, init_inventory, init_mined)
  factories = [first_factory]
  prev_time = 0
  max_geodes = 0
  while len(factories) > 0:
    (cur_time, robots, cur_inventory, cur_mined) = factories.pop(0)
    if cur_time > prev_time:
      if len(factories) > max_que_size:
        # remove the less valuable factories
        sorted_factories = sorted(factories, reverse=True, key=compare_factory)
        factories = sorted_factories[:max_que_size]
      prev_time = cur_time

    if cur_time == max_time:
      max_geodes = max(cur_mined[GEODE], max_geodes)
      continue

    new_mined = add_values(cur_mined, robots)
    new_inventory = add_values(cur_inventory, robots)
    new_factory = (cur_time+1, robots, new_inventory, new_mined)
    factories.append(new_factory)

    for p in PRODUCTS:
      if can_build_robot(blueprint.costs[p], cur_inventory):
        new_robots = robots.copy()
        new_robots[p] += 1
        new_inventory = add_values(cur_inventory, robots)
        new_inventory = subtract_values(new_inventory, blueprint.costs[p])
        new_factory = (cur_time+1, new_robots, new_inventory, new_mined)
        factories.append(new_factory)

  return max_geodes


# part-1
total = 0
blueprints = get_data()

nr = 0
for blueprint in blueprints:
  nr += 1
  geode = get_max_geode(blueprint, 24, 1000)
  print(f'{nr} / {geode}')
  total += nr * geode
print(f'part-1 total:{total}')

#part-2

nr = 0
total = 1
for blueprint in blueprints[:3]:
  nr += 1
  geode = get_max_geode(blueprint, 32, 5000)
  print(f'{nr} / {geode}')
  total *=  geode
print(f'part-1 total:{total}')


# geode = get_max_geode(blueprints[1])
# print(f'{nr} / {geode}')


# max_needed = get_max_needed(blueprint)
