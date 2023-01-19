
# read file into line array

from collections import namedtuple

def read_lines(filename, strip = True):
  with open(filename) as f:
      lines = f.readlines()
      lines = [l.strip('\n') for l in lines]
      if not strip:
        return lines
      lines = [l.strip() for l in lines]
      return lines

def signum(x):
  if x > 0:
    return 1
  elif x < 0:
    return -1
  else:
    return 0

Position = namedtuple('Position', ['x', 'y'])


class BoundingBox:
  def __init__(self):
    self.min_x = None
    self.max_x = None
    self.min_y = None
    self.max_y = None

  def __repr__(self):
    return f'{self.min_x},{self.min_y} / {self.max_x},{self.max_y}'

  def add_pos(self, x,y):
    if self.min_x == None:
      self.min_x = x
      self.max_x = x
      self.min_y = y
      self.max_y = y
    else:
      self.min_x = min(self.min_x, x)
      self.max_x = max(self.max_x, x)
      self.min_y = min(self.min_y, y)
      self.max_y = max(self.max_y, y)

  def inside(self, x, y):
    if x < self.min_x or x > self.max_x:
      return False
    if y < self.min_y or y > self.max_y:
      return False
    return True
    

def sort_interval(interval):
  l = min(interval[0], interval[1])
  r = max(interval[0], interval[1])
  return (l,r)

def is_inside( interval, c):
  l = interval[0]
  r = interval[1]
  return l <= c and c <= r

class Interval:
  def __init__(self, left, right):
    (l,r) = sort_interval((left,right))
    self.intervals = [(l,r)]

  def cut_out(self, l, r):
    new_intervals = []
    (l,r) = sort_interval((l,r))
    cut = False
    for interval in self.intervals:
      if r < interval[0] or interval[1] < l:
        # cut outside -> no changes
        new_intervals.append(interval)
        continue
      if l < interval[0] and interval[1] < r:
        # cut complete
        continue
      if is_inside(interval, l) and interval[0] < l:
        new_intervals.append((interval[0],l-1))
        cut = True
      if is_inside(interval, r) and r < interval[1]:
        new_intervals.append((r+1, interval[1]))
        cut = True
    self.intervals = new_intervals


def test_interval_1():
  iv = Interval(4, 8)
  iv.cut_out(1, 5)
  assert iv.intervals == [(6,8)]

def test_interval_2():
  iv = Interval(4, 8)
  iv.cut_out(4, 5)
  assert iv.intervals == [(6,8)]  

def test_interval_3():
  iv = Interval(4, 8)
  iv.cut_out(5, 5)
  assert iv.intervals == [(4,4),(6,8)]  

def test_interval_4():
  iv = Interval(4, 8)
  iv.cut_out(6, 9)
  assert iv.intervals == [(4,5)]  

def test_interval_5():
  iv = Interval(4, 8)
  iv.cut_out(7, 9)
  assert iv.intervals == [(4,6)]  

def test_interval_6():
  iv = Interval(4, 8)
  iv.cut_out(9, 10)
  assert iv.intervals == [(4,8)]  

def test_interval_7():
  iv = Interval(4, 8)
  iv.cut_out(1, 3)
  assert iv.intervals == [(4,8)]  

def test_interval_8():
  iv = Interval(4, 8)
  iv.cut_out(4, 8)
  assert iv.intervals == []  

def test_interval_9():
  iv = Interval(4, 8)
  iv.cut_out(1, 9)
  assert iv.intervals == []  

def test_interval_10():
  # multiple cuts
  iv = Interval(2, 8)
  iv.cut_out(3, 4)
  iv.cut_out(6, 7)
  assert iv.intervals == [(2,2),(5,5),(8,8)]  
 

test_interval_2()