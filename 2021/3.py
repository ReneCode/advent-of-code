# https://adventofcode.com/2021/day/3

numbers = []

with open('./advent_of_code_2021/3.data') as f:
    lines = f.readlines()
    numbers = [l.strip() for l in lines]

def count_one_bits(numbers, bit):
  cnt = 0
  for number in numbers:
    if number[bit] == '1':
      cnt = cnt +1
  return cnt

def most_common_bit(numbers, bit):
  half = len(numbers) / 2
  cnt = count_one_bits(numbers, bit)
  if cnt > ( len(numbers) - cnt):
    return '1'
  else:
    return '0'

def invert(bits):
  result = ''
  for b in bits:
    if b == '1':
      result = f'{result}0'
    else:
      result = f'{result}1'
  return result


bits = []
cnt_bits = len(numbers[0])
for i in range(cnt_bits):
  bits.append(most_common_bit(numbers, i))

nr_bin = ''.join(bits)
gamma_rate = int(nr_bin, 2)
nr_bin_invert = invert(nr_bin)
epsilon_rate = int(nr_bin_invert, 2)



print(gamma_rate, epsilon_rate, gamma_rate * epsilon_rate)