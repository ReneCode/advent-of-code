
# https://adventofcode.com/2021/day/22

import util

class Box:
  def __init__(self, x, y, z):
    self.x = x
    self.y = y
    self.z = z

  def __repr__(self):
    xr = f'x:{self.x[0]}-{self.x[1]}'
    yr = f'y:{self.y[0]}-{self.y[1]}'
    zr = f'z:{self.z[0]}-{self.z[1]}'
    return f'{xr} {yr} {zr}'

  def cube_dict(self, xr, yr, zr):
    xmin = max(xr[0], self.x[0])
    xmax = min(xr[1], self.x[1])
    ymin = max(yr[0], self.y[0])
    ymax = min(yr[1], self.y[1])
    zmin = max(zr[0], self.z[0])
    zmax = min(zr[1], self.z[1])
    result = {}
    for x in range(xmin, xmax+1):
      for y in range(ymin, ymax+1):
        for z in range(zmin, zmax+1):
          key = (x,y,z)
          result[key] = True
    return result

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

  def split(self, dimension, nr):
    pass

  def substraction(self, other):
    pass


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

def add_dict(a, b):
  result = a.copy()
  for key,_val in b.items():
    if result.get(key) == None:
      result[key] = True
  return result

def sub_dict(a, b):
  result = a.copy()
  for key,_val in b.items():
    if result.get(key) != None:
      del result[key]
  return result

data = read_data('./22-example-1.data')
result = {}
xrange = (-50,50)
yrange = (-50,50)
zrange = (-50,50)
for d in data:
  on = d[0]
  box = d[1]
  cubes = box.cube_dict(xrange, yrange, zrange)
  if on:
    result = add_dict(result, cubes)
  else:
    result = sub_dict(result, cubes)

print(f'finished cubes: {len(result)}')

