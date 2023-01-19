# Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
# A for Rock, B for Paper, and C for Scissors
# X for Rock, Y for Paper, and Z for Scissors.

import util

rounds_you_lost = ["AZ", "BX", "CY", ]
rounds_you_win = ["AY", "BZ", "CX"]
rounds_draw = ["AX", "BY", "CZ"]


def get_selected_point(shape):
  if shape == "X":
    return 1
  elif shape == "Y":
    return 2
  elif shape == "Z":
    return 3
  raise Exception(f'bad shape: {shape}')


def get_round_outcome(other_shape, my_shape):
  key = other_shape+my_shape
  if key in rounds_draw:
    return 3  
  if key in rounds_you_win:
    return 6
  if key in rounds_you_lost:
    return 0
  raise Exception(f'not able to get round outcome: {other_shape}:{my_shape}')

def get_needed_result(my_shape):
  if my_shape == "X":
    return "loose"
  if my_shape == "Y":
    return "draw"
  if my_shape == "Z":
    return "win"
  raise Exception(f'not able to get needed result: {my_shape}')


def get_data():
  lines = util.read_lines('./2-example.data')
  rounds = [line.split(" ") for line in lines]
  return rounds

def get_needed_shape(other_shape, needed_result):
  if needed_result == "win":
    for r in rounds_you_win:
      if r[0] == other_shape:
        return r[1]
  elif needed_result == "loose":
    for r in rounds_you_lost:
      if r[0] == other_shape:
        return r[1]
  elif needed_result == "draw":
    for r in rounds_draw:
      if r[0] == other_shape:
        return r[1]
  raise Exception(f'can not get needed shape: {other_shape} {needed_result}')

rounds = get_data()
total = 0
for r in rounds:
  other_shape = r[0]
  my_shape = r[1]
  outcome = get_round_outcome(other_shape, my_shape)
  point = get_selected_point(my_shape)
  total += (outcome + point)

print(f'total: {total}')

total_2 = 0
for r in rounds:
  other_shape = r[0]
  my_shape = r[1]
  needed_result = get_needed_result(my_shape)
  my_choose_shape = get_needed_shape(other_shape, needed_result)
  outcome = get_round_outcome(other_shape, my_choose_shape)
  point = get_selected_point(my_choose_shape)
  total_2 += (outcome + point)

print(f'total_2: {total_2}')
