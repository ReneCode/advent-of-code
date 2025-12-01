# day 1

def read_lines(filename, strip = True):
  with open(filename) as f:
      lines = f.readlines()
      lines = [l.strip('\n') for l in lines]
      if not strip:
        return lines
      lines = [l.strip() for l in lines]
      return lines


def sign(x):
  if x < 0:
    return -1
  elif x > 0:
    return 1
  else:
    return 0
  