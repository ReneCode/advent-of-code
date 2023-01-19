import util


def contains(a, x, y):
  return a >= x and a <= y

class Pair:

  def __init__(self, ra, rb):
    # convert to numbers
    # sort, so x1 <= x2
    self.a1 = min(int(ra[0]), int(ra[1]))
    self.a2 = max(int(ra[0]), int(ra[1]))
    self.b1 = min(int(rb[0]), int(rb[1]))
    self.b2 = max(int(rb[0]), int(rb[1]))

  def __repr__(self):
    return f'{self.a1}-{self.a2}:{self.b1}-{self.b2}'

  def fully_contains(self):
    if contains(self.a1, self.b1, self.b2) and contains(self.a2, self.b1, self.b2):
      return True
    if contains(self.b1, self.a1, self.a2) and contains(self.b2, self.a1, self.a2):
      return True
    return False

  def overlap(self):
    if contains(self.a1, self.b1, self.b2) or contains(self.a2, self.b1, self.b2):
      return True
    if contains(self.b1, self.a1, self.a2) or contains(self.b2, self.a1, self.a2):
      return True
    return False


def get_data():
  lines = util.read_lines('./4.data')
  pairs = []
  for line in lines:
    [a,b] = line.split(',')
    rga = a.split('-')
    rgb = b.split('-')
    pair = Pair(rga, rgb)
    pairs.append(pair)
  return pairs

pairs = get_data()
total = 0
for pair in pairs:
  fully_contains = pair.fully_contains()
  if fully_contains:
    total += 1
  
print(f'total fully contains: {total}')

total_overlap = 0
for pair in pairs:
  if pair.overlap():
    total_overlap += 1
print(f'total overlap: {total_overlap}')