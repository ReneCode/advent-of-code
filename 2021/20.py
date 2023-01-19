
# https://adventofcode.com/2021/day/20

import util

LIGHT_ON = "#"
LIGHT_OFF = "."

class Image:
  def __init__(self, lines):
    self.positions = {}
    self.col_min = 0
    self.col_max = len(lines[0])-1
    self.row_min = 0
    self.row_max = len(lines)-1
    for row,line in enumerate(lines):
      for col,val in enumerate(line):
        pos = (col,row)
        if val == LIGHT_ON:
          self.positions[pos] = True
        else:
          self.positions[pos] = False

  def output(self):
    print('------------')
    count_on = 0
    border = 0
    for row in range(self.row_min-border, self.row_max+1 + border):
      line = ""
      for col in range(self.col_min-border, self.col_max+1 + border):
        pos = (col, row)
        if self.positions.get(pos):
          line = line + LIGHT_ON
          count_on = count_on + 1
        else:
          line = line + LIGHT_OFF
      print(line)
    return count_on

  def add_border(self, val, width):
    if width == 0:
      return

    self.col_min = self.col_min-1
    self.col_max = self.col_max+1
    self.row_min = self.row_min-1
    self.row_max = self.row_max+1
    for col in range(self.col_min, self.col_max+1):
      # top
      pos = (col, self.row_min)
      self.positions[pos] = val
      # bottom
      pos = (col, self.row_max)
      self.positions[pos] = val
    for row in range(self.row_min, self.row_max+1):
      # left
      pos = (self.col_min, row)
      self.positions[pos] = val
      pos = (self.col_max, row)
      self.positions[pos] = val
    self.add_border(val, width-1)

  def get_bit_val(self, col, row):
    bits = ""
    for r in range(row-1, row+2):
      for c in range(col-1, col+2):
        pos = (c, r)
        bit = "0"
        if self.positions[pos]:
          bit = "1"
        bits = bits + bit
    val = int(bits, 2)
    return val

  def calc_next(self, algo):
    # take border val from top,left
    border_pos = (self.col_min, self.row_min)
    border_val = self.positions[border_pos]
    self.add_border(border_val, 2)
    border = 0
    result_lines = []
    for row in range(self.row_min+1, self.row_max):
      line = ""
      for col in range(self.col_min+1, self.col_max):
        bit_val = self.get_bit_val(col, row)
        result_val = algo[bit_val]
        w = LIGHT_OFF
        if result_val:
          w = LIGHT_ON
        line = line + w
      result_lines.append(line)
    return Image(result_lines)

def read_data(filename):
  lines = util.read_data(filename)
  algo_line = ""
  image_lines = []
  read_algo = True
  for line in lines:
    if line == "":
      read_algo = False
    else:
      if read_algo:
        algo_line = algo_line + line
      else:
        image_lines.append(line)
  # dict for algo
  algo = {}
  for i,val in enumerate(algo_line):
    if val == '#':
      algo[i] = True
    else:
      algo[i] = False

  return (algo, image_lines)

def get_image_boundary(image):
  col_min = 0
  col_max = 0
  row_min = 0
  row_max = 0
  for pos in image:
    (col, row) = pos
    col_min = min(col_min, col)
    col_max = max(col_max, col)
    row_min = min(row_min, row)
    row_max = max(row_max, row)
  return (col_min, col_max, row_min, row_max)





(algo, image_lines) = read_data('./20.data')
image = Image(image_lines)
image.add_border(False, 1)
# print(algo)
image.output()
for n in range(50):
  image = image.calc_next(algo)
  count = image.output()
# image = calc(image, algo)
# output(image)
# image = calc(image, algo)
# count = output(image)
print(f'pixels on: {count}')

