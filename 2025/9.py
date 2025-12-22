# 2025 / day9


with open("9.data") as f:
  lines = f.readlines()
lines = [line.strip("\n") for line in lines]

points = [[int(x) for x in line.split(",")] for line in lines]



def calc_area(idx1, idx2, points):
  p1 = points[idx1]
  p2 = points[idx2]
  area = (abs(p1[0] - p2[0]) + 1) * (abs(p1[1] - p2[1]) + 1)
  return area

def part1():
  max_area = 0
  for i in range(len(points)):
    for j in range(i + 1, len(points)):
      area = calc_area(i, j, points)
      if area > max_area:
        max_area = area
  print("Part1, Max area:", max_area)



def print_points(points):
  min_x = min(p[0] for p in points)
  max_x = max(p[0] for p in points)
  min_y = min(p[1] for p in points)
  max_y = max(p[1] for p in points)


  delta_x = max_x - min_x
  delta_y = max_y - min_y

  print(f"X: {min_x} to {max_x} (delta {delta_x})")
  print(f"Y: {min_y} to {max_y} (delta {delta_y})")

  exit()

  margin = 1
  for y in range(min_y - margin, max_y + margin + 1):
    row = ""
    for x in range(min_x - margin, max_x + margin + 1):
      if [x, y] in points:
        row += "#"
      else:
        row += "."
    print(row)


def is_point_in_range(r1, r2, p1, p2):
  min_r = min(r1, r2)
  max_r = max(r1, r2)

  min_p = min(p1, p2)
  max_p = max(p1, p2)
  if max_p <= min_r or min_p >= max_r:
    return False
  else:
    return True

def get_horizontal_and_vertical_lines(points):
  horizontal_lines = []
  vertical_lines = []
  count_points = len(points)
  for i in range(count_points):
    p1 = points[i]
    j = i+1
    if j >= count_points:
      j = 0
      
    p2 = points[j]
    horizontal = p1[1] == p2[1]   # y-axis same
    vertical = p1[0] == p2[0]     # x-axis same

    if not (horizontal or vertical):
      raise Exception("Non axis-aligned line detected")

    if horizontal:
      y = p1[1]
      horizontal_lines.append((min(p1[0], p2[0]), max(p1[0], p2[0]), y))
    elif vertical:
      x = p1[0]
      vertical_lines.append((min(p1[1], p2[1]), max(p1[1], p2[1]),x))
    else:
      raise Exception("Non axis-aligned line detected")
    
  # sort horizontal lines by the y value
  # x1, x2, y
  horizontal_lines.sort(key=lambda line: line[2])

  # sort vertical lines by the x value
  # y1, y2, x
  vertical_lines.sort(key=lambda line: line[2])

  return (horizontal_lines, vertical_lines)



def get_check_points(p1, p2):
  min_x = min(p1[0], p2[0])
  max_x = max(p1[0], p2[0])
  min_y = min(p1[1], p2[1])
  max_y = max(p1[1], p2[1])

  # one pixel inside area
  check_points = [
    (min_x + 1, min_y + 1),   # top left
    (max_x - 1, min_y + 1),   # top right
    (min_x + 1, max_y - 1),   # bottom left
    (max_x - 1, max_y - 1),   # bottom right
  ]
  return check_points

def test_get_check_points():
  p1 = (14, 11)
  p2 = (7, 8)
  check_points = get_check_points(p1, p2)
  assert check_points == [
    (8,9), (13,9),
    (8,10), (13,10)
  ]


def check_area_inside_polygon(idx1, idx2, points, horizontal_lines, vertical_lines):
  check_points = get_check_points(points[idx1], points[idx2])

  counts = []
  for check_point in check_points:
    x = check_point[0]
    y = check_point[1]

    count = 0
    # Cast ray rightwards (increasing x)
    for (y1, y2, line_x) in vertical_lines:
      if x < line_x:
        if y1 < y < y2:
          count += 1
    counts.append(count)
    if count % 2 == 0:
      return False
  if counts[0] != counts[1] or counts[2] != counts[3]:
    # inconsistent counts on top left and top right 
    # or bottom left and bottom right
    return False

  counts = []
  for check_point in check_points:  # bottom left and bottom right
    x = check_point[0]
    y = check_point[1]
    count = 0
    # Cast ray downwards (increasing y)
    for (x1, x2, line_y) in horizontal_lines:
      if y < line_y:
        if x1 < x < x2:
          count += 1
    counts.append(count)
    if count % 2 == 0:
      return False
  if counts[0] != counts[2] or counts[1] != counts[3]:
    # inconsistent counts on top left and bottom left 
    # or top right and bottom right
    return False
  
  # all checks passed
  return True



def part2():
  max_area = 0
  idx1 = 0
  idx2= 0
  doit= False
  (horizontal_lines, vertical_lines) = get_horizontal_and_vertical_lines(points)
  for i in range(len(points)):
    for j in range(i + 1, len(points)):
      area = calc_area(i, j, points)
      if area > max_area:

        if check_area_inside_polygon(i, j, points, horizontal_lines, vertical_lines):
          print("  Valid", area)
          max_area = area
          continue
  print("Part2, Max area:", max_area)


# part1()
part2()