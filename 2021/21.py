
# https://adventofcode.com/2021/day/21

import util

DICE_MAX_NR = 100
BOARD_MAX_POS = 10
GAME_WIN_POINTS = 1000

class Player:
  def __init__(self, nr, pos):
    self.nr = nr
    self.pos = pos
    self.points = 0

  def __repr__(self):
    return f'Player {self.nr} on position {self.pos} has {self.points} points'

  def walk(self, steps):
    self.pos = (self.pos + steps) % BOARD_MAX_POS
    if self.pos == 0:
      self.pos = 10
    self.points = self.points + self.pos
    return self.points

class Dice:
  def __init__(self):
    self.rolls = 0
    self.last_nr = 0

  def roll(self, count):
    result = []
    for c in range(count):
      self.rolls = self.rolls +1
      self.last_nr = self.last_nr +1
      if self.last_nr > DICE_MAX_NR:
        self.last_nr = 1
      result.append(self.last_nr)
    return tuple(result)

def read_data(filename):
  lines = util.read_data(filename)
  players = []
  for line in lines:
    tok = line.split(" ")
    nr = int(tok[1])
    pos = int(tok[4])
    player = Player(nr, pos)
    players.append(player)
  return players

players = read_data("./21.data")
dice = Dice()
current_player_idx = 0
play = True
while play:
  (nr1, nr2, nr3) = dice.roll(3) 
  nr = nr1 + nr2 + nr3
  current_player = players[current_player_idx]
  points = current_player.walk(nr)
  print(f'Player {current_player.nr} rolls {nr1}+{nr2}+{nr3} and moves to space {current_player.pos} for a total score of {current_player.points}')
  current_player_idx = (current_player_idx + 1) % 2
  if points >= GAME_WIN_POINTS:
    score = dice.rolls * players[current_player_idx].points
    print(f'finished - score: {score}')
    play = False

