from collections import namedtuple
import util

Position = namedtuple("Position", ['x','y'])

TILE_FREE = "."
TILE_WALL = "#"
TILE_OUT = " "


DIR_RIGHT = 0
DIR_DOWN = 1
DIR_LEFT = 2
DIR_UP = 3

POS_DELTA = {
  DIR_UP:    (0,-1),
  DIR_RIGHT: (1,0),
  DIR_DOWN:  (0,1),
  DIR_LEFT:  (-1,0)
}


def get_data():
  lines = util.read_lines("./22.data", False)
  board = []
  commands = ""
  read_board = True
  max_len = 0
  for line in lines:
    if line.strip() == "":
      read_board = False
      continue
    if read_board:
      max_len = max(max_len, len(line))
      board.append(line)
    else:
      commands = []
      snr = ""
      for c in line:
        if c.isdecimal():
          snr += c
        else:
          commands.append(("MOVE", int(snr)))
          snr = ""
          commands.append(("TURN", c))
      commands.append(("MOVE", int(snr)))

  next_board = []
  for line in board:
    if len(line) < max_len:
      line += TILE_OUT * (max_len - len(line))
    next_board.append(line)
  return (next_board, commands)

def get_start_pos(board):
  line = board[0]
  for x in range(len(line)):
    if line[x] == TILE_FREE:
      return Position(x,0)
  raise Exception(f'no start position found')


def turn(direction, turn):
  if turn == "R":
    return (direction +1) % 4
  elif turn == "L":
    return (direction -1) % 4
  raise Exception(f'bad turn: {turn}')


def get_next_pos(pos, direction):
  delta = POS_DELTA[direction]
  return Position(pos.x + delta[0], pos.y + delta[1])

def adjust_pos(board, pos, direction):
  len_y = len(board)
  outside = pos.y >= len_y or \
            pos.y < 0 or \
            pos.x >= len(board[pos.y]) or \
            pos.x < 0 or \
            board[pos.y][pos.x] == TILE_OUT
  if outside and direction == DIR_UP:
    for y in reversed(range(len_y)):
      if board[y][pos.x] != TILE_OUT:
        return Position(pos.x,y)
    raise Exception(f'ups a')
  if outside and direction == DIR_DOWN:
    for y in range(len_y):
      if board[y][pos.x] != TILE_OUT:
        return Position(pos.x,y)
    raise Exception(f'ups b')
  if outside and direction == DIR_LEFT:
    for x in reversed(range(len(board[pos.y]))):
      if board[pos.y][x] != TILE_OUT:
        return Position(x, pos.y)
    raise Exception(f'ups c')
  if outside and direction == DIR_RIGHT:
    for x in range(len(board[pos.y])):
      if board[pos.y][x] != TILE_OUT:
        return Position(x, pos.y)
    raise Exception(f'ups d')
  return pos


def move(board, count, pos, direction):
  for i in range(count):
    next_pos = get_next_pos(pos, direction)
    next_pos = adjust_pos(board, next_pos, direction)
    new_tile = board[next_pos.y][next_pos.x]
    if new_tile == TILE_WALL:
      return pos
    # valid move      
    pos = next_pos
  return pos


####################


def adjust_cube_pos(cube, pos, direction):
  len_y = len(board)
  outside = pos.y >= len_y or \
            pos.y < 0 or \
            pos.x >= len(cube[pos.y]) or \
            pos.x < 0 or \
            cube[pos.y][pos.x] == TILE_OUT

  if not outside:
    return (pos, direction)

  x = None
  y = None
  if outside and direction == DIR_UP:
    if pos.x in range(0,50):
      direction = DIR_RIGHT
      x = 50
      y = 50 + pos.x
    elif pos.x in range(50,100):
      direction = DIR_RIGHT
      x = 0
      y = 150 + (pos.x - 50)
    elif pos.x in range(100,150):
      direction = DIR_UP
      x = pos.x - 100
      y = 199
    else:
      raise Exception("ups dir_up")

  elif outside and direction == DIR_RIGHT:
    if pos.y in range(0,50):
      direction = DIR_LEFT
      x = 99
      y = 100 + (49 - pos.y)
    elif pos.y in range(50,100):
      direction = DIR_UP
      x = 100 + (pos.y - 50)
      y = 49
    elif pos.y in range(100,150):
      direction = DIR_LEFT
      x = 149
      y = 149 - pos.y
    elif pos.y in range(150,200):
      direction = DIR_UP
      x = 50 + (pos.y - 150)
      y = 149
    else:
      raise Exception(f"dir_right fault: {pos}")

  elif outside and direction == DIR_DOWN:
    if pos.x in range(0,50):
      direction = DIR_DOWN
      x = 100 + pos.x
      y = 0
    elif pos.x in range(50,100):
      direction = DIR_LEFT
      x = 49
      y = 150 + (pos.x - 50)
    elif pos.x in range(100,150):
      direction = DIR_LEFT
      x = 99
      y = 50 + (pos.x - 100)
    else:
      raise Exception("dir_down")

  elif outside and direction == DIR_LEFT:
    if pos.y in range(0,50):
      direction = DIR_RIGHT
      x = 0
      y = 149 - pos.y
    elif pos.y in range(50,100):
      direction = DIR_DOWN
      x = pos.y - 50
      y = 100
    elif pos.y in range(100,150):
      direction = DIR_RIGHT
      x = 50
      y = 149 - pos.y
    elif pos.y in range(150,200):
      direction = DIR_DOWN
      x = 50 + (pos.y - 150)
      y = 0
    else:
      raise Exception("dir_left")

  if x < 0 or y < 0 or y >= len(board) or x >= len(board[0]):
    print(f"bad pos: {x} {y}")
    raise Exception(f"bad pos: {x} {y}")
  pos = Position(x,y)
  return (pos, direction)

#############  

def move_on_cube(cube, count, pos, direction):
  for i in range(count):
    next_pos = get_next_pos(pos, direction)
    (next_pos, next_direction) = adjust_cube_pos(cube, next_pos, direction)
    new_tile = cube[next_pos.y][next_pos.x]
    if new_tile == TILE_WALL:
      return (pos, direction)
    # valid move      
    pos = next_pos
    direction = next_direction
    # print(pos, direction)
  return (pos, direction)

(board, commands) = get_data()

def part_1():
  pos = get_start_pos(board)
  direction = DIR_RIGHT
  print(f'start: {pos} {direction}')
  nr = 0
  for cmd in commands:
    if nr == 10:
      a = 42
    nr += 1
    if cmd[0] == "MOVE":
      pos = move(board, cmd[1], pos, direction)
    elif cmd[0] == "TURN":
      direction = turn(direction, cmd[1])
    else:
      raise Exception(f'bad cmd: {cmd[0]}')
    print(f'after {nr} {cmd}: {pos} {direction}')

  result_col = pos.x +1
  result_row = pos.y +1
  result = 1000 * result_row + 4 * result_col + direction
  print(f'part-1 result:{result}')
    

# part-2
def part_2():
  print("\nstart part-2")
  (cube, commands) = get_data()
  pos = get_start_pos(cube)
  direction = DIR_RIGHT
  nr = 0
  print(f'start: {pos} {direction}')
  for cmd in commands:
    nr += 1
    if cmd[0] == "MOVE":
      (pos, direction) = move_on_cube(cube, cmd[1], pos, direction)
    elif cmd[0] == "TURN":
      direction = turn(direction, cmd[1])
    else:
      raise Exception(f'bad cmd:{cmd}')
    print(f'after {nr} {cmd}: {pos} {direction}')
  result_col = pos.x +1
  result_row = pos.y +1
  result = 1000 * result_row + 4 * result_col + direction
  print(f'part-2 result:{result}')
      
      
part_2()


(cube, commands) = get_data()
empty_cube = []
for line in cube:
  line = line.replace(TILE_WALL, TILE_FREE)
  empty_cube.append(line)


def test_1():
  (pos, direction) = move_on_cube(board, 1, Position(50,0), DIR_UP)
  assert pos == (0, 150)
  assert direction == DIR_RIGHT
def test_1a():
  (pos, direction) = move_on_cube(board, 1, Position(99,0), DIR_UP)
  assert pos == (0, 199)
  assert direction == DIR_RIGHT
def test_1b():
  (pos, direction) = move_on_cube(board, 1, Position(100,0), DIR_UP)
  assert pos == (0, 199)
  assert direction == DIR_UP

def test_2():
  (pos, direction) = move_on_cube(board, 1, Position(50,0), DIR_LEFT)
  assert pos == (0, 149)
  assert direction == DIR_RIGHT
def test_3():
  (pos, direction) = move_on_cube(board, 1, Position(149,0), DIR_UP)
  assert pos == (49, 199)  
  assert direction == DIR_UP
def test_4():
  (pos, direction) = move_on_cube(board, 1, Position(149,0), DIR_RIGHT)
  assert pos == (149, 0)  # BLOCK
  assert direction == DIR_RIGHT
def test_5():
  (pos, direction) = move_on_cube(board, 1, Position(149,1), DIR_RIGHT)
  assert pos == (99,148)
  assert direction == DIR_LEFT



def test_round():
  pos = Position(60, 20)
  (pos, direction) = move_on_cube(empty_cube, 200, pos, DIR_RIGHT)
  assert pos == (60,20)
  assert direction == DIR_RIGHT

  pos = Position(60, 20)
  (pos, direction) = move_on_cube(empty_cube, 200, pos, DIR_LEFT)
  assert pos == (60,20)
  assert direction == DIR_LEFT


  (pos, direction) = move_on_cube(empty_cube, 200, pos, DIR_UP)
  assert pos == (60,20)
  assert direction == DIR_UP

  pos = Position(60,20)
  (pos, direction) = move_on_cube(empty_cube, 200, pos, DIR_DOWN)
  assert pos == (60, 20)
  assert direction == DIR_DOWN

  pos = Position(60, 70)
  (pos, direction) = move_on_cube(empty_cube, 200, pos, DIR_RIGHT)
  assert pos == (60 ,70)
  assert direction == DIR_RIGHT
  (pos, direction) = move_on_cube(empty_cube, 200, pos, DIR_LEFT)
  assert pos == (60 ,70)
  assert direction == DIR_LEFT


# test_round()