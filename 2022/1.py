

import util

class Elve:
  total = 0
  calories = []

  def __repr__(self):
    return f'{self.total} / {self.calories}'

def get_data():
  elves = []
  lines = util.read_lines('./1.data')
  lines.append('')
  current_elve = Elve()
  for line in lines:
    if line == '':
      elves.append(current_elve)
      current_elve = Elve()
    else:
      calory = int(line)
      current_elve.calories.append(calory)
      current_elve.total += calory
  return elves
    
elves = get_data()
totals = [e.total for e in elves]

max_total = max(totals)
print(f'Max total: {max_total}')

totals = sorted(totals, reverse=True)
max_three_total = sum(totals[:3])
print(f'Max three total: {max_three_total}')
