import re

def find_max_number(s):
  for nr in reversed(range(10, 100)):
    nr_str = str(nr)
    a, b = nr_str[0], nr_str[1]
    pattern = f'{a}.*{b}'
    if re.search(pattern, s):
      print(nr)
      return nr


with open("3.data") as f:
  lines = f.readlines()
  lines = [l.strip('\n') for l in lines]

total_sum = 0
for line in lines:
  nr = find_max_number(line)
  total_sum += nr

print("Total sum:", total_sum)