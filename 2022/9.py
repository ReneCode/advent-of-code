
from collections import namedtuple
import util

Position = namedtuple('Position', ['x', 'y'])
Move = namedtuple('Move', ['direction', 'count'])

def get_data():
  lines = util.read_lines('./9.data')
  moves = []
  for line in lines:
    tok = line.split()
    move = Move(tok[0], int(tok[1]))
    moves.append(move)
  return moves

def calc_next_positions(pos, move):
  new_positions = []
  for i in range(move.count):
    if move.direction == "U":
      pos = (Position(pos.x, pos.y+1))
    elif move.direction == "D":
      pos = (Position(pos.x, pos.y-1))
    elif move.direction == "L":
      pos = (Position(pos.x-1, pos.y))
    elif move.direction == "R":
      pos = (Position(pos.x+1, pos.y))
    else:
      raise Exception(f'bad move direction: {move.direction}')
    new_positions.append(pos)
  return new_positions

def follow_head(head, pos):
  dx = head.x - pos.x
  dy = head.y - pos.y
  if abs(dx) <= 1 and abs(dy) <= 1:
    # tail keeps positon
    return pos
  return Position(pos.x + util.signum(dx), pos.y + util.signum(dy))


moves = get_data()
tail_positions = set()
tail = Position(0,0)
head = Position(0,0)
tail_positions.add(tail)
moves = get_data()
for move in moves:
  next_positions = calc_next_positions(head, move)
  for head in next_positions:
    tail = follow_head(head, tail)
    tail_positions.add(tail)
    # print(head, tail)

print(f'count tail positions: {len(tail_positions)}')

# rope is an array of positions
# rope[0] = head
# ropt[-1] = tail
ROPE_LENGTH = 10
rope = []
for i in range(ROPE_LENGTH):
  rope.append(Position(0,0))
tail_positions = set()
tail_positions.add(rope[-1])
for move in moves:
  head = rope[0]
  next_positions = calc_next_positions(head, move)
  for head in next_positions:
    rope[0] = head
    for i in range(1,ROPE_LENGTH):
      p1 = rope[i-1]
      p2 = rope[i]
      next_pos = follow_head(p1, p2)
      rope[i] = next_pos
    tail = rope[-1]
    tail_positions.add(tail)

print(f'rope length: {ROPE_LENGTH} count tail positions: {len(tail_positions)}')