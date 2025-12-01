
with open("1.data") as f:
  lines = f.readlines()
  lines = [l.strip('\n') for l in lines]


current_position= 50
count_zero = 0

for line in lines:
  direction, distance = line[0], line[1:]
  distance = int(distance)
  match direction:
    case 'R':
      current_position += distance
    case 'L':
      current_position -= distance
    case _:
      raise ValueError(f"Unknown direction {direction}")
  current_position = current_position % 100
  if current_position == 0:
    count_zero += 1
    
print(f"Final position: {current_position}, passed 0 a total of {count_zero} times")