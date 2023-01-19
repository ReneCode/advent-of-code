
# https://adventofcode.com/2021/day/11

import util

def read_data():
  lines = util.read_data('./11.data')
  data = []
  for line in lines:
    row = [int(n) for n in line]
    data.append(row)
  return data


def get_neighbours_points(data, pt):
  xlen = len(data[0])
  ylen = len(data)
  (x,y) = pt
  xs = [x]
  ys = [y]
  if x > 0:
    xs.append(x-1)
  if x+1 < xlen:
    xs.append(x+1)
  if y > 0:
    ys.append(y-1)
  if y+1 < ylen:
    ys.append(y+1)
  neighbours = []
  for x in xs:
    for y in ys:
      neighbour_pt = (x,y)
      if neighbour_pt != pt:
        neighbours.append(neighbour_pt)
  return neighbours


def inc_value(data, pt):
  (x,y) = pt
  data[y][x] = data[y][x] +1

def clear_value(data, pt):
  (x,y) = pt
  data[y][x] = 0

def get_value(data, pt):
  (x,y) = pt
  return data[y][x]

def get_all_points(data):
  xlen = len(data[0])
  ylen = len(data)
  all_pts = []
  for y in range(ylen):
    for x in range(xlen):
      all_pts.append((x,y))
  return all_pts


def check_for_flashes(data, flash_points):
  all_pt = get_all_points(data)
  for pt in all_pt:
    if not pt in flash_points and get_value(data, pt) > 9:
      flash(data, pt, flash_points)

def flash(data, pt, flash_points):
  flash_points.append(pt)
  neighbours = get_neighbours_points(data, pt)
  for neighbour_pt in neighbours:
    inc_value(data, neighbour_pt)

  check_for_flashes(data, flash_points)


def step(data):
  # increase all by 1
  all_pts = get_all_points(data)
  for pt in all_pts:
    inc_value(data, pt)

  flash_points = []
  check_for_flashes(data, flash_points)
  for pt in flash_points:
    clear_value(data, pt)

  return len(flash_points)

def output(data):
  for row in data:
    line = ''.join([str(n) for n in row])  
    print(line)


def part_1():
  data = read_data()
  output(data)
  total_flashes = 0
  count_steps = 100
  for i in range(count_steps):
    print(f'----- After step {i+1}')
    flashes = step(data)
    total_flashes = total_flashes + flashes
    output(data)

  print(f'--- total flashes {total_flashes}')

def part_2():
  data = read_data()
  output(data)
  max_count = len(data) * len(data[0])
  i = 0
  while True:
    i = i+1
    flashes = step(data)
    print(f'----- After step {i}')
    output(data)
    if max_count == flashes:
      break

part_2()