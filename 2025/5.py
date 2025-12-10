

with open("5.data") as f:
  lines = f.readlines()
  lines = [line.strip('\n') for line in lines]

ranges = []
ids = []

read_range = True
for line in lines:
  if line == "":
    read_range = False
    continue
  if read_range:
    parts = line.split("-")
    ranges.append((int(parts[0]), int(parts[1])))
  else:
    ids.append(int(line))

def part1():
  total_valid = 0
  for id in ids:
    for r in ranges:
      if r[0] <= id <= r[1]:
        total_valid += 1
        break
  print("part1:", total_valid)


def expand_range(r1, r2):
  if r1[1] < r2[0] or r1[0] > r2[1]:
    return None
  return (min(r1[0], r2[0]), max(r1[1], r2[1]))


def test_expand_range():
  assert expand_range((5,10), (8,12)) == (5,12)
  assert expand_range((5,10), (1,5)) == (1,10)
  assert expand_range((5,10), (11,15)) == None
  assert expand_range((5,10), (1,4)) == None


def count_ids_in_range(r):
  return r[1] - r[0] + 1

def part2():
  new_ranges = []

  for r in ranges:
    add_range = r
    merged = True
    while merged:
      merged = False
      for i in range(len(new_ranges)):
        expanded = expand_range(add_range, new_ranges[i])
        if expanded is not None:
          add_range = expanded
          # Remove the old range
          del new_ranges[i]
          merged = True
          break
      if not merged:
        new_ranges.append(add_range)

  total_ids = sum([count_ids_in_range(r) for r in new_ranges  ])
  print("merged ranges:", total_ids)

# part1()
part2()