from collections import namedtuple
import util

Position = namedtuple("Position", ["x", "y"])

ELVE = "#"
EMPTY = "."

"""
 round step 1 for all, then step 2 for all

  1: consider 8 directions N, NE, E, SE, S, ...
    if all 8 targets are not ELVE than do nothing
    else
      1. if no N, NW, NE => move N
      2. if no S, SW, SE => move S
      3. if no W, SW, NW => move W
      4. if no E, SE, NE => move E
     (after one round that checking changes order, 1. goes to the end)

  2: move to target, if no conflict on that target, otherwise do nothing


  make 10 rounds
  bounding box containing all elves
  count the free spaces in that box  
"""


CHECK_NORTH = 0   # y -1
CHECK_SOUTH = 1   # y +1
CHECK_WEST = 2    # x -1
CHECK_EAST = 3    # x +1

class Elve:
  def __init__(self, pos):
    self.pos = pos
    self.new_pos = None

  def set_new_pos(self, pos):
    self.new_pos = pos

  

def get_data():
  lines = util.read_lines("./23.data")
  positions = set()
  for y in range(len(lines)):
    line = lines[y]
    for x in range(len(line)):
      w = line[x]
      if w == ELVE:
        pos = Position(x,y)
        positions.add(pos)
  return positions

def is_free_around(positions, pos):
  for dx in range(-1, 2):
    for dy in range(-1, 2):
      if dx == 0 and dy == 0:
        continue
      check_pos = Position(pos.x +dx, pos.y+dy)
      if check_pos in positions:
        return False
  return True


def get_three_positions(pos, check):
  if check == CHECK_NORTH:
    return [
      Position(pos.x-1, pos.y-1), # NW
      Position(pos.x, pos.y-1),   # N
      Position(pos.x+1, pos.y-1)  # NE
    ]
  if check == CHECK_SOUTH:
    return [
      Position(pos.x-1, pos.y+1), # SW
      Position(pos.x, pos.y+1),   # S
      Position(pos.x+1, pos.y+1)  # SE
    ]
  if check == CHECK_WEST:
    return [
      Position(pos.x-1, pos.y-1), # NW
      Position(pos.x-1, pos.y),   # W
      Position(pos.x-1, pos.y+1)  # SW
    ]
  if check == CHECK_EAST:
    return [
      Position(pos.x+1, pos.y-1), # NE
      Position(pos.x+1, pos.y),   # E
      Position(pos.x+1, pos.y+1)  # SE
    ]
  raise Exception(f'ups bad check {check}')

def get_new_pos(positions, current_pos, start_check):
  check = start_check
  for i in range(4):
    target_positions = get_three_positions(current_pos, check)
    is_free = True
    for p in target_positions:
      if p in positions:
        is_free = False
        break
    if is_free:
      return target_positions[1]
    check = (check +1) % 4
  # no free target found - stay on the old position
  return current_pos    

def print_positions(positions):
  bbox = util.BoundingBox()
  for pos in positions:
    bbox.add_pos(pos.x, pos.y)
  for y in range(bbox.min_y-1, bbox.max_y+2):
    line = ""
    for x in range(bbox.min_x-1, bbox.max_x+2):
      pos = Position(x,y)
      if pos in positions:
        line += ELVE
      else:
        line += EMPTY
    print(line)


def calc_round(positions, check_direction):
  elves = [Elve(pos) for pos in positions]
  # first: consider new position
  for elve in elves:
    if is_free_around(positions, elve.pos):
      elve.set_new_pos(elve.pos)
    else:
      new_pos = get_new_pos(positions, elve.pos, check_direction)
      elve.set_new_pos(new_pos)

  # second: move to new position
  # count elves per new position
  all_targets = {}
  for elve in elves:
    new_pos = elve.new_pos
    if all_targets.get(new_pos) == None:
      all_targets[new_pos] = 1
    else:
      all_targets[new_pos] += 1

  new_positions = set()
  for elve in elves:
    new_pos = elve.new_pos
    if all_targets.get(new_pos) == 1:
      # only one elve want to go to that new position
      new_positions.add(new_pos)
    else:
      # otherwise keep the old position
      new_positions.add(elve.pos)
  positions = new_positions
  # next round start with next direction
  check_direction = (check_direction + 1) % 4

  return (positions, check_direction)


def part_1():
  positions = get_data()
  start_check_direction = CHECK_NORTH
  print_positions(positions)
  check_direction = start_check_direction
  for r in range(10):
    (positions, check_direction) = calc_round(positions, check_direction)
    print(f'\n---- after finished round: {r+1} ----')
    print_positions(positions)

  bbox = util.BoundingBox()
  for pos in positions:
    bbox.add_pos(pos.x, pos.y)
  total_empty = 0
  for x in range(bbox.min_x, bbox.max_x+1):
    for y in range(bbox.min_y, bbox.max_y+1):
      pos = Position(x,y)
      if not pos in positions:
        total_empty += 1
  print(f'part-1 total empty tiles: {total_empty}')


def part_2():
  positions = get_data()
  start_check_direction = CHECK_NORTH
  print_positions(positions)
  check_direction = start_check_direction
  round = 0
  while True:
    round += 1
    previous_positions = positions
    (positions, check_direction) = calc_round(positions, check_direction)
    print(f'\n---- after finished round: {round} ----')
    print_positions(positions)
    if previous_positions == positions:
      break
  print(f'part-2 stable position after {round} rounds')


# part_1()
part_2()