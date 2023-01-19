
import util

def get_data():
  lines = util.read_lines('./6.data')
  return lines[0]



def get_start_of_package_marker_pos(stream, marker_len):
  chars = []
  nr = 1
  for w in stream:
    chars.append(w)
    if len(chars) == marker_len:
      unique_chars = set(chars)
      if len(unique_chars) == marker_len:
        return nr
      # remove first element for next round
      chars = chars[1:]
    nr += 1
  raise Exception(f'no start marker found')

datastream = get_data()
marker_len = 4
marker_pos = get_start_of_package_marker_pos(datastream, marker_len)
print(f'start of marker with marker length: {marker_len} => {marker_pos}')

marker_len = 14
marker_pos = get_start_of_package_marker_pos(datastream, marker_len)
print(f'start of marker with marker length: {marker_len} => {marker_pos}')