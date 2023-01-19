from collections import namedtuple
import util

Position = namedtuple("Position", ['x','y','z'])

def get_data():
  lines = util.read_lines("./18.data")
  positions = []
  for line in lines:
    tok = line.split(",")
    pos = Position(int(tok[0]), int(tok[1]), int(tok[2]))
    positions.append(pos)
  return positions

def has_common_surface(a, b):
  if a.x == b.x and a.y == b.y and abs(a.z - b.z) == 1:
    return True
  if a.x == b.x and abs(a.y - b.y) == 1 and a.z == b.z:
    return True
  if abs(a.x - b.x) == 1 and a.y == b.y and a.z == b.z:
    return True
  return False


def get_sides(positions):
  tested = []

  sides = None
  for pos in positions:
    if sides == None:
      tested.append(pos)
      sides = 6
    else:
      sides += 6
      for other in tested:
        if has_common_surface(pos, other):
          sides -= 2
      tested.append(pos)
  return sides


positions = get_data()

# part-1
sides = get_sides(positions)
print(f'part-1 sides: {sides}')

# part-2

# get bounding box
min_x = None
min_y = None
min_z = None
max_x = None
max_y = None
max_z = None
for pos in positions:
  if min_x == None:
    min_x = pos.x
    max_x = pos.x
    min_y = pos.y
    max_y = pos.y
    min_z = pos.z
    max_z = pos.z
  else:
    min_x = min(min_x, pos.x)
    max_x = max(max_x, pos.x)
    min_y = min(min_y, pos.y)
    max_y = max(max_y, pos.y)
    min_z = min(min_z, pos.z)
    max_z = max(max_z, pos.z)


def get_neighbours(pos, min_x, min_y, min_z, max_x, max_y, max_z ):
  (x,y,z) = pos
  neighbours = []
  if x>min_x:
    neighbours.append(Position(x-1,y,z))
  if x<max_x:
    neighbours.append(Position(x+1,y,z))
  if y>min_y:
    neighbours.append(Position(x,y-1,z))
  if y<max_y:
    neighbours.append(Position(x,y+1,z))
  if z>min_z:
    neighbours.append(Position(x,y,z-1))
  if z<max_z:
    neighbours.append(Position(x,y,z+1))
  return neighbours

# flood fill
min_x -= 1 
max_x += 1 
min_y -= 1
max_y += 1
min_z -= 1
max_z += 1

cube_set = set(positions)

sides_from_outside = 0
outside_cubes = set()
que = []
pos = Position(min_x, min_y, min_z)
que.append(pos)
visited = set()
# flood fill
# if reached a cube (cube_set) than it will reach it comming from exact on side
# do not make redundant neighbour test - use visited
while len(que) > 0:
  pos = que.pop(0)
  (x,y,z) = pos
  neighbours = get_neighbours(pos, min_x, min_y, min_z, max_x, max_y, max_z)
  for neighbour in neighbours:
    if not neighbour in visited:
      if neighbour in cube_set:
        sides_from_outside += 1
      else:
        que.append(neighbour)
        visited.add(neighbour)

print(f'part-2 sides: {sides_from_outside}')

