import util

class Move:
  def __init__(self, times, source, destination):
    self.times = int(times)
    self.source = int(source)
    self.destination = int(destination)

  def __repr__(self):
    return f'move {self.times} from {self.source} to {self.destination}'

def get_data():
  lines = util.read_lines('./5.data', strip=False)
  stacks = {}
  moves = []
  parse_stacks = True
  for line in lines:
    if line == '':
      parse_stacks = False
      continue

    if parse_stacks:
      for idx in range(len(line)):
        crate = line[idx]
        if crate.isnumeric():
          # bottom line of stacks
          break
        if crate == ' ' or crate == '[' or crate == ']':
          continue
        stack_nr = (idx // 4) + 1
        if stack_nr not in stacks:
          stacks[stack_nr] = []
        stacks[stack_nr].insert(0, crate)
    else:
      # parse moves
      tok = line.split()
      moves.append(Move(tok[1], tok[3], tok[5]))

  return (stacks, moves)


def move_crate(stacks, move):
  for i in range(move.times):
    crate = stacks[move.source].pop()
    stacks[move.destination].append(crate)

def move_crate_cratemover9001(stacks, move):
  cnt = move.times
  cut = stacks[move.source][-cnt:]
  del stacks[move.source][-cnt:]
  stacks[move.destination].extend(cut)


(stacks, moves) = get_data()

for move in moves:
  move_crate(stacks, move)

# atention: dictionary is not sorted
result = [stacks[nr][-1] for nr in sorted(stacks.keys())]
result = "".join(result)
print(f'part-1: {result}')



(stacks, moves) = get_data()

for move in moves:
  move_crate_cratemover9001(stacks, move)

# atention: dictionary is not sorted
result = [stacks[nr][-1] for nr in sorted(stacks.keys())]
result = "".join(result)
print(f'part-2: {result}')

