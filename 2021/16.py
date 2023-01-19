
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


def get_data(binary, versions):
  (binary, count_bits, version, typeid) = get_version_typeid(binary)
  versions.append(version)
  print(f'version: {version} typeid:{typeid}')
  if typeid == 4:
    (binary, count_bits_literal, literal_value) = get_literal_value(binary)
    count_bits = count_bits + count_bits_literal
    print(f'literal_value: {literal_value}')
  else:
    (binary, length_typeid) = read_bits_as_number(binary, 1)
    count_bits = count_bits +1
    if length_typeid == 0:
      (binary, length_of_subpackages) = read_bits_as_number(binary, 15)
      count_bits = count_bits +15
      print(f'bit-length subpackages {length_of_subpackages}')
      while length_of_subpackages > 0:
        (binary, count_bits_data) = get_data(binary, versions)
        length_of_subpackages = length_of_subpackages - count_bits_data
        count_bits = count_bits + count_bits_data
    else:
      (binary, number_of_subpackages) = read_bits_as_number(binary, 11)
      count_bits = count_bits +11
      print(f'number of subpackages {number_of_subpackages}')
      for i in range(number_of_subpackages):
        (binary, count_bits_data) = get_data(binary, versions)
        count_bits = count_bits + count_bits_data



  return (binary, count_bits)


stream = 'D2FE28'
stream = '38006F45291200'
stream = 'EE00D40C823060'

# 00111000000000000110111101000101001010010001001000000000

stream = '8A004A801A8002F478'
stream = '620080001611562C8802118E34'
stream = 'C0015000016115A2E0802F182340'
stream = 'A0016C880162017C3686B18A3D4780'

lines = util.read_data('./16.data')
stream = lines[0]

binary = get_binary(stream)
print(binary)
versions = []
get_data(binary, versions)
print(versions, sum(versions))







