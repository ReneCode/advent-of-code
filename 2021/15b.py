
# https://adventofcode.com/2021/day/15

import sys
import util
import random

def read_data():
  board = []
  lines = util.read_data('./15.data')
  for line in lines:
    row = [int(n) for n in line]
    board.append(row)
  return board

def out_board(board):
  print('-----------')
  for row in board:
    line = ''.join([str(n) for n in row])
    print(line)



def test_blowup_board():
  board = [[1,2],[3,4]]
  result = blowup_board(board, 3)
  print(result)
  assert len(result) == 6
  assert len(result[0]) == 6
  row = result[0]
  # assert row == [1,2,2,3,3,4]


def test_insert_board():
  board = [[1,2],[3,8]]
  expect_2 = [ 
    [1,2,2,3],
    [3,8,4,9],
    [2,3,3,4],
    [4,9,5,1]
  ]
  expect_3 = [ 
    [1,2,2,3,3,4],
    [3,8,4,9,5,1],
    [2,3,3,4,4,5],
    [4,9,5,1,6,2],
    [3,4,4,5,5,6],
    [5,1,6,2,7,3]
  ]
  assert blowup_board(board, 2) == expect_2
  assert blowup_board(board, 3) == expect_3


def test_inc_val():
  assert inc_val(7, 3) == 1

def test_copy_row():
  assert copy_row([1,2], 3, 0) == [1,2,2,3,3,4]
  assert copy_row([1,2], 3, 1) == [2,3,3,4,4,5]
  assert copy_row([1,2], 3, 7) == [8,9,9,1,1,2]


def inc_val(v, times):
  val = v
  for i in range(times):
    val = val +1
    if val > 9:
      val = 1
  return val

def copy_row(row, times, inc):
  row_len = len(row)
  target = []
  for i in range(times):
    for v in row:
      val = inc_val(v, i + inc)
      target.append(val)
  return target



def blowup_board(board, times):
  xlen = len(board[0])
  ylen = len(board)
  result_board = []
  for i_board in range(times):
    for row in board:
      result_row = copy_row(row, times, i_board)
      result_board.append(result_row)
  return result_board

class TableItem:
  def __init__(self, pt):
    self.pt = pt
    self.cost = None
    self.prev_point = None

  def set_cost(self, cost, prev_point):
    if self.cost == None or self.cost > cost:
      self.cost = cost
      self.prev_point = prev_point
    else:
      raise Exception(f'do not set to higher costs on pt: {pt}')


class DijkstraWayFinder:
  def __init__(self, board):
    self.ways = []
    self.waiting_points = []
    self.visited_points = {}
    self.table = {}
    self.board = board
    (xlen, ylen) = self.calc_shape()
    self.start_pt = (0,0)
    self.end_pt = (xlen-1, ylen-1)


  def get_table_item(self, pt):
    return self.table[pt]

  def init_table(self):
    (xlen, ylen) = self.calc_shape()
    for y in range(ylen):
      for x in range(xlen):
        pt = (x,y)
        self.table[pt] = TableItem(pt)
    item = self.table[self.start_pt]
    item.set_cost(0, None)

  def get_risk(self, pt):
    if pt == self.start_pt:
      return 0
    (x,y) = pt
    return self.board[y][x]

  def calc_shape(self):
    return (len(self.board[0]), len(self.board))

  def get_next_points(self, pt):
    (xlen, ylen) = self.calc_shape()
    pts = []
    (x,y) = pt
    if x > 0:
      pts.append((x-1,y))
    if y > 0:
      pts.append((x,y-1))
    if x+1 < xlen:
      pts.append((x+1,y))
    if y+1 < ylen:
      pts.append((x,y+1))
    return pts

  def add_waiting(self, pt):
    if not pt in self.waiting_points:
      self.waiting_points.append(pt)

  def remove_waiting(self, pt):
    self.waiting_points.remove(pt)

  def add_visited(self, pt):
    self.visited_points[pt] = True

  def is_visited(self, pt):
    return self.visited_points.get(pt) != None

  def get_waiting(self):
    # get waiting with min costs
    items = [self.get_table_item(pt) for pt in self.waiting_points]
    costs = [item.cost for item in items]
    min_cost = min(costs)
    for item in items:
      if min_cost == item.cost:
        return item
    raise Exception(f'no min-cost item found')


  def calc(self):
    cnt = 0
    self.add_waiting(self.start_pt)
    while len(self.waiting_points) > 0:
      cnt = cnt+1
      if cnt % 1000 == 0:
        print(f'cnt: {cnt}')
      current_item = self.get_waiting()
      next_points = self.get_next_points(current_item.pt)
      for next_point in next_points:
        if not self.is_visited(next_point):
          risk = self.get_risk(next_point)
          cost = current_item.cost + risk
          next_item = self.get_table_item(next_point)
          if next_item.cost == None or next_item.cost > cost:
            next_item.set_cost(cost, current_item.pt)
          self.add_waiting(next_point)
      self.add_visited(current_item.pt)
      self.remove_waiting(current_item.pt)
          


  def out_way(self, way):
    (xlen, ylen) = self.calc_shape()
    print('-------------------')
    for y in range(ylen):
      line = ''
      for x in range(xlen):
        pt = (x,y)
        if pt in way:
          line = line + '#'
        else:
          line = line + '.'
      print(line)

  def calc_risk(self, way):
    total_risk = 0
    for pt in way:
      risk = self.get_risk(pt)
      total_risk = total_risk + risk
    return total_risk

  def calc_min_risk_of_all_ways(self):
    min_risk = None
    min_way = None
    for way in self.ways:
      total_risk = self.calc_risk(way)
      if min_risk == None or (total_risk < min_risk):
        min_risk = total_risk
        min_way = way
    return (min_risk, min_way)


sys.setrecursionlimit(500*500+100)
board = read_data()
board = blowup_board(board, 5)
# out_board(board)
print('==== start ====')
way_finder = DijkstraWayFinder(board)
way_finder.init_table()
way_finder.calc()
# ways = way_finder.calc_all_ways()
# (min_risk, min_way) = way_finder.calc_min_risk_of_all_ways()
last_item = way_finder.get_table_item(way_finder.end_pt)
print(f'============= {last_item.cost}')
# way_finder.out_way(min_way)
# print(len(way_finder.ways), min_risk)
