# https://adventofcode.com/2021/day/2

commands = []

with open('./advent_of_code_2021/2.data') as f:
    lines = f.readlines()
    commands = [l.strip() for l in lines]


hpos = 0
depth = 0
for cmd in commands:
  tok = cmd.split()
  match tok:
    case ['down', nr]:
      depth = depth + int(nr)
    case ['up', nr]:
      depth = depth - int(nr)
    case ['forward', nr]:
      hpos = hpos + int(nr)
      
print(hpos, depth, hpos * depth)

print("-----")




hpos = 0
depth = 0
aim = 0
for cmd in commands:
  tok = cmd.split()
  match tok:
    case ['down', nr]:
      # depth = depth + int(nr)
      aim = aim + int(nr)
    case ['up', nr]:
      # depth = depth - int(nr)
      aim = aim - int(nr)
    case ['forward', nr]:
      hpos = hpos + int(nr)
      depth = depth + aim * int(nr)

print(hpos, depth, hpos * depth)
