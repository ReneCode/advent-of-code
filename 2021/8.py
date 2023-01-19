
# https://adventofcode.com/2021/day/8

import util


def read_data():
  lines = util.read_data('./8.data')
  result = []
  for line in lines:
    tok = line.split("|")
    patterns = tok[0].split()
    output = tok[1].split()
    result.append( (patterns, output) )
  return result

def count_pattern_with_len(signals, lengths):
  count = 0
  for signal in signals:
    length = len(signal)
    if length in lengths:
      count = count +1
  return count


def get_segment_length(digits):
  result = []
  lenghts = { 0:6, 1:2, 2:5, 3:4, 4:4, 5:5, 6:6, 7:3, 8:7, 9:6 }
  for digit in digits:
    result.append(lenghts.get(digit))
  return result



data = read_data()
sum_count = 0
segment_lengths = get_segment_length([1,4,7,8])
for dt in data:
  count = count_pattern_with_len(dt[1],segment_lengths)
  print(f'{dt[1]} {count}')
  sum_count = sum_count + count

print(sum_count)



# print(data)
