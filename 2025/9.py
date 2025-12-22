# 2025 / day9


with open("9.testdata") as f:
  lines = f.readlines()
lines = [line.strip("\n") for line in lines]

points = [[int(x) for x in line.split(",")] for line in lines]



def calc_area(idx1, idx2, points):
  p1 = points[idx1]
  p2 = points[idx2]
  area = abs(p1[0] - p2[0]+1) * abs(p1[1] - p2[1]+1)
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

def is_valid_area(idx1, idx2, points):
  p1 = points[idx1]
  p2 = points[idx2]
  min_x = min(p1[0], p2[0])
  max_x = max(p1[0], p2[0])
  min_y = min(p1[1], p2[1])
  max_y = max(p1[1], p2[1])

  count_points = len(points)
  for i in range(count_points):
    p1 = points[i]
    j = i+1
    if j >= count_points:
      j = 0
      
    p2 = points[j]
    horizontal = p1[1] == p2[1]
    vertical = p1[0] == p2[0]
    if horizontal: 
      # is line inside area? on the y axis
      if min_y < p1[1] < max_y:
        if is_point_in_range(min_x, max_x, p1[0], p2[0]):
        # if (min_x < p1[0] < max_x) or (min_x < p2[0] < max_x):
            return False
    elif vertical:
      if min_x < p1[0] < max_x:
        if is_point_in_range(min_y, max_y, p1[1], p2[1]):
        # if (min_y < p1[1] < max_y) or (min_y < p2[1] < max_y):
            return False
    else:
      raise Exception("Non axis-aligned line detected")

  return True


def part2():
  max_area = 0
  for i in range(len(points)):
    for j in range(i + 1, len(points)):
      area = calc_area(i, j, points)
      if area >= max_area:
        print("Checking area between points", points[i], points[j])
        if is_valid_area(i, j, points):
          print("  Valid", area)
          max_area = area
        else:
          print("  Invalid")
  print("Part2, Max area:", max_area)

# part1()
part2()