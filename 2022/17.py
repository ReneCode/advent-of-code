from collections import namedtuple
import util

Position = namedtuple('Position', ['x', 'y'])


MOVE_LEFT = "<"
MOVE_RIGHT = ">"



def read_data():
  air_lines = util.read_lines("./17.data")

  shapes = []
  shape_lines = util.read_lines("./17-shapes.data")
  # read from bottom to top to increment y-pos while reading
  shape_lines = list(reversed(shape_lines))
  shape_lines.append("")   # easier to finish last shape
  cur_shape = set()
  
  cur_y = 0
  for line in shape_lines:
    if line == "":
      shapes.append(cur_shape)
      cur_shape = set()
      cur_y = 0
    else:
      for x in range(0, len(line)):
        w = line[x]
        if w == "#":
          cur_shape.add(Position(x,cur_y))
      cur_y += 1

  # top of file should be first shape
  shapes = list(reversed(shapes))
  return (air_lines[0], shapes)



def shape_translate(shape, delta):
  new_shape = set()
  for pos in shape:
    new_pos = Position(pos.x + delta.x, pos.y +delta.y)
    new_shape.add(new_pos)
  return new_shape

def get_shape_max_y(shape):
  max_y = None
  for pos in shape:
    if max_y == None:
      max_y = pos.y
    else:
      max_y = max(max_y, pos.y)
  return max_y

def get_watermark(chamber):
  watermark = [-1 for i in range(0, 7)]
  for pos in chamber:
    if pos.x >= 0 and pos.x <= 6:
      watermark[pos.x] = max(watermark[pos.x], pos.y)
  min_y = min(watermark)
  watermark = [i-min_y for i in watermark]
  return watermark

(air_pattern, shapes) = read_data()

chamber = set()
chamber_width = 7
shape_start_x = 2
shape_start_y_offset = 3
y_max = 0
visited = {}

# ground of chamber
for x in range(0, chamber_width+2):
  chamber.add(Position(x-1, -1))

# first move left/right
# then fall down  (rest, if no more down possible)

rocks = 2022

idx_air = 0
idx_shape = 0
for n_shape in range(1, rocks+1):
  idx_shape = idx_shape % 5
  shape = shapes[idx_shape]
  idx_shape += 1
  
  shape_height = 1 +get_shape_max_y(shape)
  # add border left and right / full height of shape
  for dy in range(0, shape_height+shape_start_y_offset):
    chamber.add(Position(-1,y_max+dy))
    chamber.add(Position(chamber_width,y_max+dy))

  shape = shape_translate(shape, Position(shape_start_x, y_max+shape_start_y_offset))
  shape_falling = True
  while shape_falling:
    cur_air = air_pattern[idx_air]
    idx_air += 1
    idx_air = idx_air % len(air_pattern)
    new_shape = None
    if cur_air == MOVE_RIGHT:
      new_shape = shape_translate(shape, Position(1,0))
    elif cur_air == MOVE_LEFT:
      new_shape = shape_translate(shape, Position(-1,0))
    else:
      raise Exception(f'bad air type: {cur_air}')

    overlap = chamber.intersection(new_shape)
    if len(overlap) == 0:
      shape = new_shape

    # move down
    new_shape = shape_translate(shape, Position(0,-1))
    overlap = chamber.intersection(new_shape)
    if len(overlap) == 0:
      shape = new_shape
    else:
      if n_shape % 1000 == 0:
        print(f'set shape {n_shape}')
      for pos in shape:
        chamber.add(pos)
      shape_falling = False
      y_shape_max = get_shape_max_y(shape)
      y_max = max(y_max, y_shape_max+1)
      watermark = get_watermark(chamber)
      watermark = [str(i) for i in watermark]
      key = (idx_shape, idx_air, ".".join(watermark))
      value = (n_shape,y_max)
      if visited.get(key) == None:
        visited[key] = [value]
      else:
        visited[key].append(value)
        # print out for part-2
        print(f"**** repeat **** n:{n_shape} {key}: {visited[key]}")


print(f'y max: {y_max}')

"""
calculation for part-2

look for periods where shape-idx, wind-idx and watermark are the same
periode length = number of shapes (dx)
periode y-delta = increase of y_max during that periode

1. get periode dx and dy

2. get value of y_max on nshape > periode-length  => start value

3. calc number of periodes that completely fits into the rest of shapes
   (count_shape - start_count_shape) // dx

4. the rest_shapes of that equation = (count_shape - start_count_shape) mod dx 

5. get y_max value of start_count_shape + rest_shapes

6. add (count_full_periods * dy)







"""
