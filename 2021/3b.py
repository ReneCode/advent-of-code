# https://adventofcode.com/2021/day/3

org_numbers = []

with open('./advent_of_code_2021/3.data') as f:
    lines = f.readlines()
    org_numbers = [l.strip() for l in lines]

def count_bits(numbers, bit, val):
  cnt = 0
  for number in numbers:
    if number[bit] == val:
      cnt = cnt +1
  return cnt

def most_common_bit(numbers, bit):
  half = len(numbers) / 2
  cnt = count_bits(numbers, bit, '1')
  if cnt >= ( len(numbers) - cnt):
    return '1'
  else:
    return '0'

def less_common_bit(numbers, bit):
  half = len(numbers) / 2
  cnt = count_bits(numbers, bit, '0')
  if cnt <= ( len(numbers) - cnt):
    return '0'
  else:
    return '1'

def invert(bits):
  result = ''
  for b in bits:
    if b == '1':
      result = f'{result}0'
    else:
      result = f'{result}1'
  return result


numbers = org_numbers
bit_len = len(numbers[0])
oxygen_generator_rating = 0
for bit in range(bit_len):
  mcb = most_common_bit(numbers, bit)
  numbers = [n for n in numbers if n[bit] == mcb]
  if len(numbers) == 1:
    oxygen_generator_rating = numbers[0]
    break

oxygen_generator_rating = int(oxygen_generator_rating, 2)
print(oxygen_generator_rating)

numbers = org_numbers
c02_scrubber_rating = 0
for bit in range(bit_len):
  lcb = less_common_bit(numbers, bit)
  numbers = [n for n in numbers if n[bit] == lcb]
  if len(numbers) == 1:
    c02_scrubber_rating = numbers[0]
    break

c02_scrubber_rating = int(c02_scrubber_rating, 2)
print(c02_scrubber_rating)
life_support_rating = oxygen_generator_rating * c02_scrubber_rating
print(life_support_rating)