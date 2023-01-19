
# https://adventofcode.com/2021/day/16


import util

def hex_to_bin(str):
  result = ''
  for w in str:
    integer = int(w, 16)
    binary = format(integer, '0>4b')
    result = result + binary
  return result

def read_bits(binary, n):
  s = binary[:n]
  return (binary[n:], s)

def get_binary(str):
  binary = hex_to_bin(str)
  return binary

def read_bits_as_number(binary, n):
  (binary, nr) = read_bits(binary, n)
  nr = int(nr, 2)
  return (binary, nr)

def get_version_typeid(binary):
  (binary, version) = read_bits(binary, 3)
  version = int(version,2)
  (binary, typeid) = read_bits(binary, 3)
  typeid = int(typeid,2)
  count_bits = 6
  return (binary, count_bits, version, typeid)


def get_literal_value(binary):
  cont = True
  literal_value = ''
  count_bits = 0
  while cont:
    (binary, cont_bit) = read_bits(binary, 1)
    count_bits = count_bits +1
    (binary, value_bits) = read_bits(binary, 4)
    count_bits = count_bits +4
    literal_value = literal_value + value_bits
    cont = cont_bit == '1'
  literal_value = int(literal_value,2)
  # (binary, three_zero) = read_bits(binary, 3)
  return (binary, count_bits, literal_value)



def get_operator(binary):
  (binary, bit_len_subpackages) = read_bits(binary, 15)
  bit_len_subpackages = int(bit_len_subpackages, 2)
  return (binary, length_typeid, bit_len_subpackages)


def get_data(binary, values):
  (binary, count_bits, version, typeid) = get_version_typeid(binary)
  # print(f'version: {version} typeid:{typeid}')
  if typeid == 4:
    # value package
    (binary, count_bits_literal, literal_value) = get_literal_value(binary)
    count_bits = count_bits + count_bits_literal
    # print(f'literal_value: {literal_value}')
    values.append(literal_value)
  else:
    # operator package
    (binary, length_typeid) = read_bits_as_number(binary, 1)
    count_bits = count_bits +1
    operator_values = []
    if length_typeid == 0:
      (binary, length_of_subpackages) = read_bits_as_number(binary, 15)
      count_bits = count_bits +15
      # print(f'bit-length subpackages {length_of_subpackages}')
      while length_of_subpackages > 0:
        (binary, count_bits_data) = get_data(binary, operator_values)
        length_of_subpackages = length_of_subpackages - count_bits_data
        count_bits = count_bits + count_bits_data
    else:
      (binary, number_of_subpackages) = read_bits_as_number(binary, 11)
      count_bits = count_bits +11
      # print(f'number of subpackages {number_of_subpackages}')
      for i in range(number_of_subpackages):
        (binary, count_bits_data) = get_data(binary, operator_values)
        count_bits = count_bits + count_bits_data
    
    print(f'operator typeid {typeid} values {operator_values}')
    if typeid == 0:
      # sum
      operator_result = sum(operator_values)
    elif typeid == 1:
      # multiply
      operator_result = 1
      for v in operator_values:
        operator_result = operator_result * v
    elif typeid == 2:
      # minimum
      operator_result = min(operator_values)
    elif typeid == 3:
      # maximum
      operator_result = max(operator_values)
    elif typeid == 5:
      # >
      operator_result = 0 + (operator_values[0] > operator_values[1])
    elif typeid == 6:
      # <
      operator_result = 0 + (operator_values[0] < operator_values[1])
    elif typeid == 7:
      # ==
      operator_result = 0 + (operator_values[0] == operator_values[1])



      
    values.append(operator_result)

  return (binary, count_bits)


stream = 'C200B40A82'
stream = '04005AC33890'
stream = '880086C3E88112'
stream = 'CE00C43D881120'


stream = 'D8005AC2A8F0'
stream = 'F600BC2D8F'
stream = '9C005AC2F8F0'
stream = '9C0141080250320F1802104A08'

lines = util.read_data('./16.data')
stream = lines[0]

binary = get_binary(stream)
print(binary)
values = []
get_data(binary, values)
print(f'result: {values}')







