
import util

def parse_lines(lines):
  cmds = []
  for line in lines:
    coords = line.split("->")
    start_coord = [int(n) for n in coords[0].split(",")]
    end_coord = [int(n) for n in coords[1].split(",")]
    cmds.append([start_coord, end_coord])
  return cmds

def remove_diagonal(cmds):
  result = []
  for cmd in cmds:
    start = cmd[0]
    end = cmd[1]
    if start[0] == end[0] or start[1] == end[1]:
      result.append(cmd)
  return result

def get_max_dimension(cmds):
  max_x = 0
  max_y = 0
  for cmd in cmds:
    start = cmd[0]
    end = cmd[1]
    max_x = max(max_x, start[0])
    max_x = max(max_x, end[0])
    max_y = max(max_y, start[1])
    max_y = max(max_y, end[1])
  return (max_x, max_y)

def debug_board(board):
  for row in board:
    vals = [str(n) for n in row]
    
    line = "".join(vals)
    line = line.replace("0", ".")
    print(line)

def add_cmd(board, cmd):
  [x0, y0] = cmd[0]
  [x1, y1] = cmd[1]

  cnt_x = x1 - x0
  cnt_y = y1 - y0
  cnt = max(abs(cnt_x), abs(cnt_y))
  dx = int(cnt_x / cnt)
  dy = int(cnt_y / cnt)
  x = x0
  y = y0
  for i in range(cnt+1):
    board[y][x] = board[y][x] + 1
    x = x + dx
    y = y + dy

def count_more_than(board, nr):
  cnt = 0
  for row in board:
    for val in row:
      if val > nr:
        cnt = cnt +1
  return cnt



# advent_of_code_2021/5-example.data
input_data = util.read_data('./5.data')
cmds = parse_lines(input_data)
# only for part a
# cmds = remove_diagonal(cmds)
(max_x, max_y) = get_max_dimension(cmds)
board = [ [0]*(max_x+1) for _ in range(max_y+1) ]


for cmd in cmds:
  add_cmd(board, cmd)


result = count_more_than(board, 1)

# print(cmds)
debug_board(board)
print(result)





example=result ="""
  0123456789
0 .......1..
1 ..1....1..
2 ..1....1..
3 .......1..
4 .112111211
5 ..........
6 ..........
7 ..........
8 ..........
9 222111....
"""