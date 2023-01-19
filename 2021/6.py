# https://adventofcode.com/2021/day/6

import util


class Factory:
  def __init__(self, start_timers):
    self.fishes = []
    for timer in start_timers:
      self.create(timer)

  def create(self, nr):
    fish = Fish(nr)
    self.fishes.append(fish)

  def tick(self):
    all_fishes = self.fishes.copy()
    for fish in all_fishes:
      fish.tick(self)

  def print(self):
    numbers = [str(f.timer) for f in self.fishes]
    print(numbers)


class Fish:
  def __init__(self, nr):
    self.timer = nr

  def tick(self, factory):
    if self.timer > 0:
      self.timer = self.timer -1
      return False
    self.timer = 6
    factory.create(8)
    return True



input_data = util.read_data('./6.data')
start_timer = [int(n) for n in input_data[0].split(',')]
factory = Factory(start_timer)
factory.print()

days = 80

for i in range(days):
  factory.tick()
  print(f'After day {i+1}: {len(factory.fishes)}')

  # factory.print()