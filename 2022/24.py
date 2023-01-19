from collections import namedtuple
import util

Position = namedtuple("Position", ["x", "y"])

WIND_UP = "^"
WIND_RIGHT = ">"
WIND_LEFT = "<"
WIND_DOWN = "v"
TILE_FREE = "."


class Field:
  def __init__(self, positions, bbox, start, end):
    self.positions = positions
    self.bbox = bbox
    self.start = start
    self.end = end
    self.minute = 0

  def get_next_wind_pos(self, cur_pos, wind):
    pos = None
    if wind == WIND_DOWN:
      pos = Position(cur_pos.x, cur_pos.y+1)
      if pos.y > self.bbox.max_y:
        pos = Position(pos.x, self.bbox.min_y)
    elif wind == WIND_UP:
      pos = Position(cur_pos.x, cur_pos.y-1)
      if pos.y < self.bbox.min_y:
        pos = Position(pos.x, self.bbox.max_y)
    elif wind == WIND_RIGHT:
      pos = Position(cur_pos.x+1, cur_pos.y)
      if pos.x > self.bbox.max_x:
        pos = Position(self.bbox.min_x, pos.y)
    elif wind == WIND_LEFT:
      pos = Position(cur_pos.x-1, cur_pos.y)
      if pos.x < self.bbox.min_x:
        pos = Position(self.bbox.max_x, pos.y)
    else:
      raise Exception("bad wind:" + wind)
    return pos

  def calc_next_positions(self):
    self.minute += 1
    print(f'new field for minute {self.minute}')
    new_positions = {}
    for pos, winds in self.positions.items():
      for wind in winds:
        new_pos = self.get_next_wind_pos(pos, wind)
        if new_positions.get(new_pos) != None:
          new_positions[new_pos].append(wind)
        else:
          new_positions[new_pos] = [wind]
    self.positions = new_positions



class Board:
  def __init__(self, me, field):
    self.field = field
    self.me = me
    self.finished = False
    self.kill = False

  def print(self):
    for y in range(0, self.field.bbox.max_y+2):
      line = ""
      for x in range(0, self.field.bbox.max_x+2):
        out = "."
        pos = Position(x,y)
        if pos == self.me:
          out = "E"
        elif pos == self.field.start or pos == self.field.end:
          out = "."
        elif not self.field.bbox.inside(x,y):
          out = "#"
        else:
          if self.field.positions.get(pos) == None:
            out = "."
          else:
            winds = self.field.positions[pos]
            if len(winds) == 1:
              out = winds[0]
            else:
              out = str(len(winds))
        line += out
      print(line)

  def get_next_me(self, goal_end):
    next_pos = [
      self.me,
      Position(self.me.x,self.me.y-1),
      Position(self.me.x,self.me.y+1),
      Position(self.me.x-1,self.me.y),
      Position(self.me.x+1,self.me.y)
    ]
    result = []
    for pos in next_pos:
      if pos == goal_end:
        # final position !
        return [pos]
      if pos == self.me or self.field.bbox.inside(pos.x, pos.y):
        if self.field.positions.get(pos) == None:
          result.append(pos)
    return result

  def set_me(self, me, goal):
    self.me = me
    if self.me == goal:
      self.finished = True
      self.print()
      print(" ************** finished ")
      return True
    return False

def get_data():
  lines = util.read_lines("./24.data")
  x_len = len(lines[0])
  y_len = len(lines)
  x = lines[0].index(TILE_FREE)
  start = Position(x,0)
  x = lines[y_len-1].index(TILE_FREE)
  end = Position(x, y_len-1)
  positions = {}
  for y in range(1, y_len-1):
    for x in range(1, x_len-1):
      if lines[y][x] != TILE_FREE:
        pos = Position(x,y)
        positions[pos] = [lines[y][x]] 
  bbox = util.BoundingBox()
  bbox.add_pos(1,1)
  bbox.add_pos(x_len-2, y_len-2)
  field = Field(positions, bbox, start, end)
  return field

def remove_boards_with_same_position(boards):
  me_positions = set()
  new_boards = []
  for board in boards:
    me_pos = board.me
    if not me_pos in me_positions:
      me_positions.add(me_pos)
      new_boards.append(board)
  return new_boards


field = get_data()

print('==== start ====')
goals = [ 
  (field.start, field.end),
  ###############################
  # next two lines are for part-2 
  # (field.end, field.start),
  # (field.start, field.end),
]
minute = 0
while len(goals) > 0:
  (goal_start, goal_end) = goals.pop(0)
  first_board = Board(goal_start, field)
  first_board.print()
  boards = [first_board]
  finished = False
  while not finished:
    minute += 1
    prev_count = len(boards)
    boards = [b for b in boards if not b.kill ]
    boards = remove_boards_with_same_position(boards)
    print(f'Minute: {minute}, reduce boards from {prev_count} => {len(boards)}')
    field.calc_next_positions()
    new_boards = []
    for board in boards:
      next_free_positions = board.get_next_me(goal_end)
      if len(next_free_positions) == 0:
        board.kill = True
      for i in range(len(next_free_positions)):
        if i == 0:
          if board.set_me(next_free_positions[i], goal_end):
            finished = True
            break
        else:
          new_board = Board(None, field)
          if new_board.set_me(next_free_positions[i], goal_end):
            finished = True
            break
          new_boards.append(new_board)
      # board.print()
      if finished:
        break
    boards.extend(new_boards)

print(f'finished !!!')