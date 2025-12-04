

def invalid_ids_v1(start, end):
  invalids = []
  for id in range(start, end+1):
    id_str = str(id)
    str_len = len(id_str)
    left_half = id_str[:str_len//2]
    right_half = id_str[str_len//2:]
    # print(f"  Checking ID {id}: left_half={left_half}, right_half={right_half}")
    if left_half == right_half:
      # print(f"    Invalid ID found: {id}")
      invalids.append(id)
  return invalids



def devide_string(str, count):
  str_len = len(str)
  part_len = str_len // count
  parts = []
  for i in range(count):
    part = str[i*part_len:(i+1)*part_len]
    parts.append(part)
  return parts

# pytest test
def test_devide_string():
  assert devide_string("1212", 2) == ["12", "12"]
  assert devide_string("123123123", 3) == ["123", "123", "123"]
  assert devide_string("abcdabcdabcdabcd", 4) == ["abcd", "abcd", "abcd", "abcd"]

def id_invalid(id_str):
  str_len = len(id_str)
  for parts in range(2, str_len+1):
    if str_len % parts == 0:
      parts = devide_string(id_str, parts)
      if all(part == parts[0] for part in parts):
        return True
  return False


def test_invalid_id():
  assert id_invalid("1212") == True
  assert id_invalid("123123") == True
  assert id_invalid("1234") == False
  assert id_invalid("1111") == True
  assert id_invalid("123123123") == True
  assert id_invalid("123124123") == False


def invalid_ids(start, end):
  collect_ids = []
  for id in range(start, end+1):
    id_str = str(id)
    if id_invalid(id_str):
      print(f"    Invalid ID found: {id}")
      collect_ids.append(id)
  return collect_ids



with open("2.data") as f:
  lines = f.readlines()
  lines = [l.strip('\n') for l in lines]

tokens = lines[0].split(',')
id_ranges = []
for token in tokens:
  start, end = token.split('-')
  id_ranges.append( (int(start), int(end)) )

# print(f"ID Ranges: {id_ranges}")

bad_ids = []
for id_range in id_ranges:
  start, end = id_range
  invalid = invalid_ids(start, end)
  bad_ids.extend(invalid)


total_sum = sum(bad_ids)
print(f"Total sum: {total_sum}")
