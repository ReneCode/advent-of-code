# https://adventofcode.com/2021/day/6

# use other approach - do not model each fish but a group of fishes with same age

import util

class Factory:
  def __init__(self, timer_and_count_tuples):
    self.fishGroups = []
    for tup in timer_and_count_tuples:
      (timer, count) = tup
      self.create(timer, count)

  def create(self, timer, count):
    # look for existing group 
    for group in self.fishGroups:
      if group.timer == timer:
        group.count = group.count + count
        return
    # create new group
    fish = FishGroup(timer, count)
    self.fishGroups.append(fish)

  def tick(self):
    result = []
    for fishGroup in self.fishGroups:
      res = fishGroup.tick(self)
      for r in res:
        result.append(r)

    # compress
    self.fishGroups = []
    for fishGroup in result:
      self.create(fishGroup.timer, fishGroup.count)

  def total_count(self):
    count = 0
    for group in self.fishGroups:
      count = count + group.count
    return count

  def print(self):
    result = ""
    for group in self.fishGroups:
      result = result + " / " + f'{group.timer}*{group.count}'
    print(result)

class FishGroup:
  def __init__(self, start_timer, count):
    self.timer = start_timer
    self.count = count

  def tick(self, factory):
    if self.timer > 0:
      return [ FishGroup(self.timer-1, self.count) ]

    fg1 = FishGroup(6, self.count)
    fg2 = FishGroup(8, self.count)
    return [fg1, fg2]



input_data = util.read_data('./6.data')
start_timers = [int(n) for n in input_data[0].split(',')]
timer_and_count_tupes = []
for timer in range(9):
  vals = [n for n in start_timers if n == timer]
  count = len(vals)
  if count > 0:
    timer_and_count_tupes.append( (timer, count) )


factory = Factory(timer_and_count_tupes)
factory.print()

days = 256

for i in range(days):
  factory.tick()
  print(f'After day {i+1}: {factory.total_count()}')
  factory.print()


_ = """ 
Initial state: 3,4,3,1,2
After  1 day:  2,3,2,0,1
After  2 days: 1,2,1,6,0,8
After  3 days: 0,1,0,5,6,7,8
After  4 days: 6,0,6,4,5,6,7,8,8
After  5 days: 5,6,5,3,4,5,6,7,7,8
After  6 days: 4,5,4,2,3,4,5,6,6,7
After  7 days: 3,4,3,1,2,3,4,5,5,6
After  8 days: 2,3,2,0,1,2,3,4,4,5
After  9 days: 1,2,1,6,0,1,2,3,3,4,8
After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
"""