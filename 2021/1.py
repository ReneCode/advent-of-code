
numbers = []

with open('./advent/1.data') as f:
    lines = f.readlines()
    numbers = [int(l.strip()) for l in lines]


prev = None
cnt_inc = 0
for nr in numbers:
  if prev != None:
    if nr > prev:
      cnt_inc = cnt_inc +1
  prev = nr
print(cnt_inc)