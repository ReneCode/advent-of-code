import re

def find_max_number(s):
  for nr in reversed(range(10, 100)):
    nr_str = str(nr)
    a, b = nr_str[0], nr_str[1]
    pattern = f'{a}.*{b}'
    if re.search(pattern, s):
      return nr

def part1(lines):
  total_sum = 0
  for line in lines:
    nr = find_max_number(line)
    total_sum += nr
  print("Total sum:", total_sum)



def get_max_number(line, length):
  rest_len = length
  cur_str = line
  result = []
  while rest_len > 0:
    cur_len = len(cur_str)
    for digit in range(9, -1, -1):
      letter = str(digit)
      pos = cur_str.find(letter)
      if pos != -1 and (cur_len - pos) >= rest_len:
        result.append(letter)
        cur_str = cur_str[pos+1:]
        rest_len -= 1
        break
  result_nr = int(''.join(result))
  return result_nr

def test_get_max_number():
  line = "123456789212"
  assert get_max_number(line, 3) == 922
  line = "234234234234278"
  assert get_max_number(line, 12) == 434234234278


def part2(lines):
  max_nrs = [get_max_number(line,12) for line in lines]
  total_sum = sum(max_nrs)
  print("Total sum:", total_sum)



with open("3.data") as f:
  lines = f.readlines()
  lines = [l.strip('\n') for l in lines]

# part1(lines)
part2(lines)  


