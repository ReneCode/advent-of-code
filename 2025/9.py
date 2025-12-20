# 2025 / day9


with open("9.data") as f:
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


part1()