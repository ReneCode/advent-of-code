
with open("7.data") as f:
  lines = f.readlines()

lines = [line.strip("\n") for line in lines]

SPLITTER = "^"
EMPTY = "."

def find_start(lines):
  top_line = lines[0]
  width = len(top_line)
  for i in range(width):
    if top_line[i] == "S":
      return (0,i)
  return None

def part1():
  total_count = 0
  height = len(lines)
  pos = find_start(lines)
  positions = {pos}
  while True:
    new_positions = set()
    for p in positions:
      r,c = p
      r += 1
      new_val = lines[r][c]
      if new_val == "^":
        total_count += 1
        new_positions.add((r,c-1))
        new_positions.add((r,c+1))
      else:
        new_positions.add((r,c))
    positions = new_positions
    if r == height-1:
      break
  print("Total count:", total_count)


def part2():
  # start from bottom up
  # collect path counts on a SPLITTER 
  # as a sum of left and right on a row below 

  height = len(lines)
  width = len(lines[0])
  path_counts = [[0 for _ in range(width)] for _ in range(height)]
  pos = find_start(lines)
  row = height - 1
  for w in range(width):
    path_counts[row][w] = 1

  total = 0
  for r in range(height-2, -1, -1):
    if "S" in lines[r]:
      for c in range(width):
        val = lines[r][c]
        if val == "S":
          total = path_counts[r+1][c]
          break

    if not SPLITTER in lines[r]:
      # only ..... in the line
      for c in range(width):
        if lines[r+1][c] == SPLITTER:
          # splitter below, sum counts from left and right
          path_counts[r][c] = path_counts[r+1][c-1] + path_counts[r+1][c+1]
        else:
          path_counts[r][c] = path_counts[r+1][c]
        
    else:
      for c in range(width):
        val = lines[r][c]
        if val == EMPTY:
          path_counts[r][c] = path_counts[r+1][c]
        elif val == SPLITTER:
          path_counts[r][c] = 0


  print("Total paths:", total)

# part1()
part2()