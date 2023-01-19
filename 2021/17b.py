
# https://adventofcode.com/2021/day/17



def read_target(line):
  tok = line.split(":")
  xy = tok[1].split(",")
  xvals = xy[0].split("=")[1]
  yvals = xy[1].split("=")[1]
  [x1,x2] = xvals.split("..")
  [y1,y2] = yvals.split("..")
  x1 = int(x1)
  x2 = int(x2)
  y1 = int(y1)
  y2 = int(y2)
  return ( (min(x1,x2), min(y1,y2)), (max(x1,x2), max(y1,y2)) )


def calc_velocity_x(target_left):
  sum = 0
  i = 0
  while sum < target_left:
    i = i+1
    sum = sum + i
  return i


def is_below(pta, ptb):
  return pta[0] < ptb[0]

def add_pt(pta, ptb):
  return (pta[0] + ptb[0], pta[1] + ptb[1])

def is_inside(pt, x_min, y_min, x_max, y_max):
  if pt[0] < x_min:
    return False
  if pt[0] > x_max:
    return False
  if pt[1] < y_min:
    return False
  if pt[1] > y_max:
    return False
  return True

def next_vel(vel):
  (dx, dy) = vel
  dy = dy -1
  if dx > 0:
    dx = dx -1
  elif dx < 0:
    dx = dx +1
  else:
    dx = 0
  return (dx, dy)


def can_reach_target(start_vel, x_min, y_min, x_max, y_max):
  pt = (0,0)
  vel = start_vel
  while True:
    pt = add_pt(pt, vel)
    vel = next_vel(vel)
    # print(pt)
    if is_inside(pt, x_min, y_min, x_max, y_max):
      return True
    if pt[0] > x_max:
      break
    if pt[1] < y_min:
      break
    if vel[0] == 0 and pt[0] < x_min:
      break
  return False

def calc_height(trajectory):
  max_height = 0
  for pt in trajectory:
    max_height = max(max_height, pt[1])
  return max_height

def output(trajectory, target, max_height):
  y = max_height
  (tp0, tp1) = target
  print('')
  while y >= tp0[1]:
    line = ''
    x = 0
    while x <= tp1[0]:
      pt = (x,y)

      if pt == (0,0):
        line = line + 'S'
      elif pt in trajectory:
        line = line + '#'
      elif is_inside(pt, tp0, tp1):
        line = line + "T"
      else:
        line = line + "."
      x = x+1
    print(line)
    y = y-1

def get_similar_vels(start_vel):
  (dx,dy) = start_vel
  vels = []
  for x in range(-80,81):
    for y in range(-80,81):
      vel = (dx+x, dy+y)
      if vel != start_vel:
        vels.append(vel)
  return vels

def calc_possible_velocities(start_pt, target):
  ((x_min, y_min), (x_max, y_max)) = (target[0], target[1])
  vy_max = -y_min -1
  count = 0
  for y in range(y_min, vy_max+1):
    for x in range(1, x_max+1):
      vel = (x,y)
      reach_target = can_reach_target(vel, x_min, y_min, x_max, y_max)
      if reach_target:
        count = count+1
        print(vel, count)

  return count


start_pt = (0,0)
line = 'target area: x=20..30, y=-10..-5'
line = 'target area: x=265..287, y=-103..-58'
      #  'target area: x=265..287, y=-103..-58'
target = read_target(line)
result = calc_possible_velocities(start_pt, target)
print (result)
