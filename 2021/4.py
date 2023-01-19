# https://adventofcode.com/2021/day/4

EMPTY = -1

def read_data():
  input_data = []

  with open('./advent_of_code_2021/4.data') as f:
      lines = f.readlines()
      input_data = [l.strip() for l in lines]

  drawns = [int(n) for n in input_data[0].split(',')]

  # remove draw-line
  input_data.pop(0)

  puzzles = []
  one_puzzle = []
  for line in input_data:
    if line == "":
      one_puzzle = []
      puzzles.append(one_puzzle)
    else:
      numbers = [int(n) for n in line.split()]
      one_puzzle.append(numbers)

  return (drawns, puzzles)


def mark_number(puzzles, nr):
  for puzzle in puzzles:
    for row in puzzle:
      for i in range(len(row)):
        if (row[i] == nr):
          row[i] = EMPTY


def all_empty(numbers):
  for nr in numbers:
    if nr != EMPTY:
      return False
  return True

def get_won_puzzle(puzzles):
  for puzzle in puzzles:
    for row in puzzle:
      if all_empty(row):
        return puzzle
    for idx in range(len(puzzle[0])):
      numbers = []
      for row in puzzle:
        numbers.append(row[idx])
      if all_empty(numbers):
        return puzzle
  return None

def remove_all_won_puzzles(puzzles):
  all_won = []
  while True:
    won = get_won_puzzle(puzzles)
    if won == None:
      break
    all_won.append(won)
    puzzles.remove(won)
  return all_won

def sum_puzzle(puzzle):
  sum = 0
  for row in puzzle:
    for nr in row:
      if nr != EMPTY:
        sum = sum + nr
  return sum


def exercise_a():
  (drawns, puzzles) = read_data()
  # print(draw)
  for drawn in drawns:
    mark_number(puzzles, drawn)
    won = get_won_puzzle(puzzles)
    if won != None:
      print(won)
      sum = sum_puzzle(won)
      score = sum * drawn
      print(score)
      break

def exercise_b():
  (drawns, puzzles) = read_data()
  for drawn in drawns:
    mark_number(puzzles, drawn)
    all_won = remove_all_won_puzzles(puzzles)

    if len(puzzles) == 0:
      print(all_won)
      won = all_won[0]
      sum = sum_puzzle(won)
      score = sum * drawn
      print(score)
      break

exercise_b()