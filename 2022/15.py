from collections import namedtuple

import util


Position = namedtuple('Position', ['x', 'y'])

class Sensor:
  def __init__(self, sensor_pos, beacon_pos):
    self.sensor_pos = sensor_pos
    self.beacon_pos = beacon_pos
    dx = abs(self.sensor_pos.x - self.beacon_pos.x)
    dy = abs(self.sensor_pos.y - self.beacon_pos.y)
    self.dist = dx + dy

  def __repr__(self):
    return f'sensor:{self.sensor_pos} / beacon:{self.beacon_pos} / dist:{self.dist}'

  def calc_covered_x_values(self, y):
    delta = abs(y - self.sensor_pos.y)
    if delta > self.dist:
      return []

    # min_y = self.sensor_pos.y - self.dist
    # max_y = self.sensor_pos.y + self.dist
    # if y < min_y or max_y < y:
    #   return []

    dx = abs(self.dist - delta)
    covered_x_values = [x for x in range(self.sensor_pos.x-dx, self.sensor_pos.x+dx +1)]
    return covered_x_values

  def get_covered_x_interval(self, min_x, max_x, y):
    delta = abs(y - self.sensor_pos.y)
    if delta > self.dist:
      return None

    dx = abs(self.dist - delta)
    rg = (self.sensor_pos.x-dx, self.sensor_pos.x+dx)
    return rg
    

def get_data():
  lines = util.read_lines('./15.data')
  sensors = []
  bounding_box = util.BoundingBox()
  for line in lines:
    tok = line.split(" ")
    sensor_x = int(tok[2].split("=")[1].split(",")[0])
    sensor_y = int(tok[3].split("=")[1].split(":")[0])
    beacon_x = int(tok[8].split("=")[1].split(",")[0])
    beacon_y = int(tok[9].split("=")[1])

    sensor = Sensor(Position(sensor_x, sensor_y), 
                    Position(beacon_x, beacon_y))
    sensors.append(sensor)
    bounding_box.add_pos(sensor_x, sensor_y)
    bounding_box.add_pos(beacon_x, beacon_y)

  return sensors


sensors = get_data()

total = 0
y = 2000000
# y = 10
# for x in range(bounding_box.min_x, bounding_box.max_x+1):
#   total += 1
beacon_xs = [s.beacon_pos.x for s in sensors]
min_x = min(beacon_xs)
max_x = max(beacon_xs)

def get_covered_x_values(sensors, y):
  all_x_values = set()
  for sensor in sensors:
    covered_x_values = sensor.calc_covered_x_values(y)
    all_x_values.update(set(covered_x_values))
  return all_x_values

all_x_values = get_covered_x_values(sensors, y)

for sensor in sensors:
  if sensor.sensor_pos.y == y:
    x = sensor.sensor_pos.x
    if x in all_x_values:
      all_x_values.remove(x)
  if sensor.beacon_pos.y == y:
    x = sensor.beacon_pos.x
    if x in all_x_values:
      all_x_values.remove(x)

all = sorted([x for x in all_x_values])
# print(all)

count = len(all_x_values)

print(f'part-1 free positions: {count}')

# part-2

sensors = get_data()
min_x = 0
max_x = 4000000
for y in range(min_x, max_x+1):
  interval = util.Interval(min_x, max_x)
  for sensor in sensors:
    iv = sensor.get_covered_x_interval(min_x, max_x, y)
    if iv != None:
      interval.cut_out(iv[0], iv[1])
  if len(interval.intervals) > 0:
    print(y, interval.intervals) 
    x = interval.intervals[0][0]
    tuning_frequency = x * 4000000 + y
    print(f'part-2 tuning frequency: {tuning_frequency}') 


