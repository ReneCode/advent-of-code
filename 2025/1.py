
from util import sign

with open("1.data") as f:
  lines = f.readlines()
  lines = [l.strip('\n') for l in lines]


current_position= 50
total_clicks = 0

for line in lines:
  direction, distance = line[0], line[1:]
  distance = int(distance)
  clicks = 0
  match direction:
    case 'L':
      distance = -distance
    case 'R':
      pass
    case _:
      raise ValueError(f"Unknown direction {direction}")


  step = sign(distance)
  for _ in range(abs(distance)):
    current_position = (current_position + step) % 100
    if current_position == 0:
      clicks += 1



  # next_position = current_position + distance

  # if next_position % 100 == 0:
  #   # final is on the zero
  #   if next_position >= 100:
  #     clicks = next_position // 100
  #   elif next_position <= -100:
  #     clicks = -(next_position // 100)
  #   elif next_position == 0:
  #     clicks = 1
  # else:
  #   if next_position >=0:
  #     clicks = next_position // 100
  #   else:
  #     clicks =  (next_position // 100) +1
  #   if sign(current_position) + sign(next_position) == 0:
  #     clicks += 1

  # current_position = (current_position + distance) % 100


  total_clicks += clicks
  print(f"Dial is rotated {line} to {current_position}, clicks: {clicks} times")

  # if current_position == 0:
  #   count_zero += 1
    
print(f"Final position: {current_position}, passed 0 a total of {total_clicks} times")