
ROLE = "@"
EMPTY = "."
REMOVED = "x"


with open("4.data") as f:
  lines = f.readlines()
  lines = [line.strip('\n') for line in lines]

width = len(lines[0])
height = len(lines)


def count_neighbors(x, y):
  deltas = [(-1, -1), (0, -1), (1, -1),
            (-1, 0),          (1, 0),
            (-1, 1),  (0, 1),  (1, 1)]
  count = 0
  for dx, dy in deltas:
    nx, ny = x + dx, y + dy
    if 0 <= nx < width and 0 <= ny < height:
      if lines[ny][nx] == ROLE:
        count += 1
  return count


def part1():
  liftings = 0
  for y in range(height):
    for x in range(width):
      if lines[y][x] == ROLE:
        neighbors = count_neighbors(x, y)
        if neighbors < 4:
          liftings += 1
  print("part1:",liftings)

def part2():
  count_removed = 0
  while True:
    can_remove = False
    removings = []
    for y in range(height):
      for x in range(width):
        if lines[y][x] == ROLE:
          neighbors = count_neighbors(x, y)
          if neighbors < 4:
            can_remove = True
            removings.append((x, y))

    for x, y in removings:
      lines[y] = lines[y][:x] + REMOVED + lines[y][x+1:]
      count_removed += 1

    if not can_remove:
      break

  print("part2:", count_removed)


# part1()
part2()