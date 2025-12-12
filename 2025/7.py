
with open("7.data") as f:
  lines = f.readlines()

lines = [line.strip("\n") for line in lines]

for line in lines:
  print(line)




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

part1()