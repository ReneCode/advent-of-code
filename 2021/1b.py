
numbers = []

with open('./advent_of_code_2021/1.data') as f:
    lines = f.readlines()
    numbers = [int(l.strip()) for l in lines]


prev = []
cnt_inc = 0
for nr in numbers:
  if len(prev) == 3:
    last_sum = prev[0] + prev[1] + prev[2]
    new_sum = prev[1] + prev[2] + nr
    if new_sum > last_sum:
      cnt_inc = cnt_inc +1
    prev.pop(0)
  prev.append(nr)

print(cnt_inc)