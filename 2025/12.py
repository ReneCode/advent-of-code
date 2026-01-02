# advent of code 2025 / day 12


class Shape:
  index = -1
  shape = []
  count_filled = 0

  def __init__(self):
    self.index = -1
    self.shape = []

  def __str__(self):
    return f"Shape {self.index}: {self.shape} Count: {self.count_filled}"
  
  def __repr__(self):
    return f"Shape {self.index}: {self.shape} Count: {self.count_filled}"


  def parse(self, lines):
    self.index = int(lines[0][0])
    for line in lines[1:]:
      self.shape.append(line)
      self.count_filled += line.count("#")

class Area:
  width = 0
  height = 0
  shapes = []

  def __init__(self):
    self.width = 0
    self.height = 0
    self.shapes = []

  def __str__(self):
    return f"Area: {self.width}x{self.height} Shapes: {self.shapes} "

  def __repr__(self):
    return f"Area: {self.width}x{self.height} Shapes: {self.shapes} "

  def parse(self, line):
    size, rest = line.split(": ", 1)
    self.width, self.height = [int(x) for x in size.split("x")]
    self.shapes = [int(x) for x in rest.split(" ")]


with open("12.testdata") as f:
  lines = f.readlines()
lines = [line.strip("\n") for line in lines]

shapes=[]
areas = []
read_shapes = True
shape_lines = []
for line in lines:
  if line.find("x") >= 0:
    read_shapes = False

  if read_shapes:
    if line == "":
      shape = Shape()
      shape.parse(shape_lines)
      shapes.append(shape)
      shape_lines = []
    else:
      shape_lines.append(line)
  else:
    # read areas
    area = Area()
    area.parse(line)
    areas.append(area)



print("shapes:")
for shape in shapes:
  print(shape)

print("areas:")
for area in areas:
  print(area)


def part1(shapes, areas):
  total_fit = 0
  for area in areas:
    area_area = area.width * area.height
    shape_area = 0
    for shape_index, shape_count in enumerate(area.shapes):
      shape = shapes[shape_index]
      shape_area += shape.count_filled * shape_count

    print(f"Area {area.width}x{area.height} = {area_area}, shapes filled area = {shape_area}")


    # shape_area = 





part1(shapes, areas)

