

def invalid_ids(start, end):
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
  print(f"Checking range {start}-{end}")
  invalid = invalid_ids(start, end)
  bad_ids.extend(invalid)


total_sum = sum(bad_ids)
print(f"Total sum: {total_sum}")
