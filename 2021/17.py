
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

def is_inside(pt, bottom_left, top_right):
  if pt[0] < bottom_left[0]:
    return False
  if pt[0] > top_right[0]:
    return False
  if pt[1] < bottom_left[1]:
    return False
  if pt[1] > top_right[1]:
    return False
  # print(f'pt inside:{pt}')
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


def calc_trajectory(start_pt, start_vel, target):
  (target_bottom_left, target_top_right) = (target[0], target[1])
  pt = start_pt
  vel = start_vel
  trajectrory = [pt]
  while pt[1] >= target_bottom_left[1]:
    pt = add_pt(pt, vel)
    vel = next_vel(vel)
    # print(pt)
    trajectrory.append(pt)
    if is_inside(pt, target_bottom_left, target_top_right):
      return trajectrory
  # did not hit the target
  return None

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


def calc_velocity_y(start_pt, velx, target):
  vely = 0
  max_height = 0
  max_vely = 0
  while vely < 100*velx:
    vel = (velx, vely)
    # vel = (17,-4)
    trajectory = calc_trajectory(start_pt, vel, target)
    if trajectory != None:
      height = calc_height(trajectory)
      if height >= max_height:
        # output(trajectory, target, height)
        max_height = height
        max_vely = vely
        print(f'vel: {(velx,vely)} max height: {max_height}')
    vely = vely +1
  return (max_vely, max_height)



start_pt = (0,0)
line = 'target area: x=20..30, y=-10..-5'
line = 'target area: x=265..287, y=-103..-58'
      #  'target area: x=265..287, y=-103..-58'
target = read_target(line)
velx = calc_velocity_x(target[0][0])
result = calc_velocity_y(start_pt, velx, target)
print(velx, result)
