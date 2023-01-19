
# https://adventofcode.com/2021/day/22

import util


"""
cut_out

a:    456789
b: 123
=> None

a:    456789
b: 1234
> cut:4
 rest: 56789

a:    456789
b: 12345678
> cut:45678
 rest:     9

a:    456789
b: 123456789
> cut:456789

a:    456789
b: 1234567890
> cut:456789

a:    456789
b:    4567
> cut:4567
 rest:    89

a:    456789
b:     567
> cut: 567
 rest:4   89

a:    456789
b:      67890
> cut:  6789
 rest:45  

a:    456789
b:         90
> cut:     9
 rest:45678

a:    456789
b:          0
> None

"""

# rg_a = (start_nr, stop_nr)
# rg_b = (start_nr, stop_nr)
def get_left_cut_right(rg_a, rg_b):
  if rg_b[1] < rg_a[0]:
    return None
  if rg_a[1] < rg_b[0]:
    return None
  left = None
  cut = None
  right = None
  if rg_a[0] < rg_b[0]:
    left = (rg_a[0], rg_b[0]-1)
  if rg_b[1] < rg_a[1]:
    right = (rg_b[1]+1, rg_a[1])
  cut = (max(rg_a[0], rg_b[0]), min(rg_a[1], rg_b[1]))
  return (left, cut, right)


def boxes_volume(boxes):
  volume = 0
  for b in boxes:
    volume += b.volume()
  return volume

class Box:
  def __init__(self, x, y, z):
    self.x = x
    self.y = y
    self.z = z

  def __repr__(self):
    xr = f'x:{self.x[0]}/{self.x[1]}'
    yr = f'y:{self.y[0]}/{self.y[1]}'
    zr = f'z:{self.z[0]}/{self.z[1]}'
    return f'{xr} {yr} {zr}'

  def __eq__(self, o):
    if isinstance(self, o.__class__):
      return self.x == o.x and self.y == o.y and self.z == o.z
    return False

  def intercept(self, other):
    # x
    if self.x[0] > other.x[1]:
      return False  # self is right of other
    if self.x[1] < other.x[0]:
      return False  # self is left of other
    # y
    if self.y[0] > other.y[1]:
      return False  # self is right of other
    if self.y[1] < other.y[0]:
      return False  # self is left of other
    # z
    if self.z[0] > other.z[1]:
      return False  # self is right of other
    if self.z[1] < other.z[0]:
      return False  # self is left of other
    return True

  def volume(self):
    dx = self.x[1] - self.x[0] + 1
    dy = self.y[1] - self.y[0] + 1
    dz = self.z[1] - self.z[0] + 1
    return dx * dy * dz

  def cutout(self, cut_box):
    if self.intercept(cut_box) == False:
      return [self]

    rest_boxes = []
    box = self
    result_x = get_left_cut_right(box.x, cut_box.x)
    if result_x != None:
      if result_x[0] != None:
        rest_boxes.append(Box(result_x[0], box.y, box.z))
      if result_x[2] != None:
        rest_boxes.append(Box(result_x[2], box.y, box.z))
      # this is the cutout
      box = Box(result_x[1], box.y, box.z)

    result_y = get_left_cut_right(box.y, cut_box.y)
    if result_y != None:
      if result_y[0] != None:
        rest_boxes.append(Box(box.x, result_y[0], box.z))
      if result_y[2] != None:
        rest_boxes.append(Box(box.x, result_y[2], box.z))
      # this is the cutout
      box = Box(box.x, result_y[1], box.z)

    result_z = get_left_cut_right(box.z, cut_box.z)
    if result_z != None:
      if result_z[0] != None:
        rest_boxes.append(Box(box.x, box.y, result_z[0]))
      if result_z[2] != None:
        rest_boxes.append(Box(box.x, box.y, result_z[2]))
      # this is the cutout
      box = Box(box.x, box.y, result_z[1])

    return rest_boxes


def read_range(str):
  tok = str.split("=")
  tok = tok[1].split("..")
  return (int(tok[0]), int(tok[1]))

def read_data(filename):      
  lines = util.read_data(filename)
  result = []
  for line in lines:
    tok = line.split(" ")
    on = tok[0] == 'on'
    coord = tok[1].split(',')
    x = read_range(coord[0])
    y = read_range(coord[1])
    z = read_range(coord[2])
    box = Box(x,y,z)
    result.append( (on, box))
  return result



data = read_data('./22.data')
boxes = []
nr = 0
for d in data:
  nr += 1
  on = d[0]
  box = d[1]
  print(f'processing on: {on} box: {box}')
  if len(boxes) == 0 and on:
    boxes.append(box)
  else:
    next_boxes = []
    for b in boxes:
      result = b.cutout(box)
      next_boxes.extend(result)
    boxes = next_boxes
    if on:
      boxes.append(box)


volume = 0
for b in boxes:
  volume += b.volume()
print(len(boxes), volume)    


