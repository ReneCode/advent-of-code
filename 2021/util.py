
# read file into line array

def read_data(filename):
  input_data = []

  with open(filename) as f:
      lines = f.readlines()
      input_data = [l.strip() for l in lines]

  return input_data
