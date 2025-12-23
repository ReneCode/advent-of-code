# advent of code 2025 day 10


class Machine:
  lights = []
  buttons = []
  requirements = []

  def __init__(self):
    self.lights = []
    self.buttons = []
    self.requirements = []

  def __str__(self):
    return f"Machine:\n lights={self.lights}\n buttons={self.buttons}\n requirements={self.requirements}\n"

  def __repr__(self):
    return f"Machine\n lights={self.lights}\n buttons={self.buttons}\n requirements={self.requirements}\n"


  def parse_line(self, line):
    parts = line.split(" ")
    for light in parts[0][1:-1]:
      on = light == "#"
      self.lights.append(on)

    for part in parts[1:]:
      if part.startswith("("):
        # button
        buttons = []
        for button in part[1:-1].split(","):
          buttons.append(int(button))
        self.buttons.append(buttons)
      elif part.startswith("{"):
        # requirement
        for req in part[1:-1].split(","):
          self.requirements.append(int(req))
      else:
        print("Unknown part in line:", part)
        exit()

with open("10.data") as f:
  lines = f.readlines()
lines = [line.strip("\n") for line in lines]
machines = []
for line in lines:
  machine = Machine()
  machine.parse_line(line)
  machines.append(machine)


def combinations(numbers, count):
  if count == 0:
    yield []
    return
  for i in range(len(numbers)):
    first = numbers[i]
    rest = numbers[i+1:]
    for combo in combinations(rest, count - 1):
      yield [first] + combo


def test_combinations():
  nums = [0, 1, 2]
  combs = list(combinations(nums, 1))
  print("Combinations of", nums, "choose 1:", combs)


def check_with_n_presses(machine, n):
  button_indexes = list(range(len(machine.buttons)))
  for combo in combinations(button_indexes, n):
    lights = [False] * len(machine.lights)
    for button_index in combo:
      button = machine.buttons[button_index]
      for light_index in button:
        lights[light_index] = not lights[light_index]
    equal = machine.lights == lights
    if equal:
      return n
  return -1

def find_min_button_presses(machine):
  n = 1
  while True:
    presses = check_with_n_presses(machine, n)
    if presses != -1:
      return presses
    n += 1
  return -1



def part1():
  # print(machines)
  presses = [find_min_button_presses(machine) for machine in machines]
  # print("Minimum button presses for machine:", presses)
  total = sum(presses)
  print("Part1. Sum of minimum button presses:", total)


part1()