
# https://adventofcode.com/2021/day/19

import util
import vector



def calc_deltas(positions, idx_pt):
  deltas = []
  rel_pt = positions[idx_pt]
  for i in range(len(positions)):
    if i != idx_pt:
      delta = vector.sub(positions[i], rel_pt)
      deltas.append(delta)
  return deltas

class Scanner:
  def __repr__(self):
    return f'{self.nr} / {self.positions}'

  def __init__(self, nr, positions):
    self.nr = nr
    self.positions = positions
    self.mapping = None
    self.offset = None

  def set_mapping(self, mapping):
    self.mapping = mapping

  def set_offset(self, vec):
    self.offset = vec

  def get_global_positions(self):
    return [vector.add(self.offset, map_vec(vec, self.mapping)) for vec in self.positions]

def read_data(filename):
  lines = util.read_data(filename)
  scanners = []
  scanner_nr = None
  positions = []
  for line in lines:
    if line.startswith('--- scanner '):
      tok = line.split(' ')
      scanner_nr = int(tok[2])
    elif line == '':
      scanner = Scanner(scanner_nr, positions)
      scanners.append(scanner)
      positions = []
      scanner_nr = None
    else:
      tokens = line.split(',')
      xyz = [int(n) for n in tokens]
      positions.append(tuple(xyz))
  if len(positions) > 0:
    scanner = Scanner(scanner_nr, positions)
    scanners.append(scanner)
  return scanners


def is_good_fit(vectors_a, vectors_b):
  count = 0
  minimum_count = 12-1  # 12 points => 11 deltas
  for vec_a in vectors_a:
    if vec_a in vectors_b:
      count = count +1
      if count >= minimum_count:
        return True
  return False

def sign(val):
  if val > 0:
    return 1
  if val < 0:
    return -1
  return 0

# mapping (2, -3, 1)
# => map (x,y,z) => (y,-z,x)
def map_vec(vec, mapping):
  x_idx = abs(mapping[0])-1
  y_idx = abs(mapping[1])-1
  z_idx = abs(mapping[2])-1
  x_sgn = sign(mapping[0])
  y_sgn = sign(mapping[1])
  z_sgn = sign(mapping[2])
  return (vec[x_idx]*x_sgn, vec[y_idx]*y_sgn, vec[z_idx]*z_sgn)

def invers_map_vec(vec, mapping):
  (x,y,z) = map_vec(vec, mapping)
  return (-x,-y,-z)

def get_all_mappings():
  bases = [(1,2,3), (1,3,2), (2,1,3), (2,3,1), (3,1,2), (3,2,1)]
  result = []
  for base in bases:
    for i in range(8):
      binary = format(i, '>03b')
      mapping = []
      for idx in range(3):
        if binary[idx] == '1':
          mapping.append(-base[idx])
        else:
          mapping.append(base[idx])
      result.append(tuple(mapping))
  return result


def find_similar_pos(scanner_a, scanner_b, all_mappings):
  for idx_a in range(len(scanner_a.positions)):
    mapped_positions_a = [map_vec(vec, scanner_a.mapping) for vec in scanner_a.positions]
    deltas_a = calc_deltas(mapped_positions_a, idx_a)
    for idx_b in range(len(scanner_b.positions)):
      if scanner_b.mapping != None:
        all_mappings = [scanner_b.mapping]
      for mapping in all_mappings:
        mapped_positions_b = [map_vec(vec, mapping) for vec in scanner_b.positions]
        deltas_b = calc_deltas(mapped_positions_b, idx_b)
        found = is_good_fit(deltas_a, deltas_b)
        if found:
          # with that mapping b-positions are mapped into a-system
          scanner_b.set_mapping(mapping)
          vec_a = mapped_positions_a[idx_a]
          vec_b = mapped_positions_b[idx_b]
          relative = vector.sub(vec_a, vec_b)
          b_offset = vector.add(scanner_a.offset, relative)
          scanner_b.set_offset(b_offset)
          # print(f'found {idx_a} {vec_a}, {idx_b} {vec_b} mapping:{mapping} relative:{relative} {mapped_relative}')
          return True
  return False




def check_overlap(idx):
  all_mappings = get_all_mappings()
  sa = scanners[idx]
  for i in range(len(scanners)):
    if i != idx:
      sb = scanners[i]
      if sb.offset == None:
        found = find_similar_pos(sa, sb, all_mappings)
        if found:
          print(idx, i, found)
          check_overlap(i)


scanners = read_data('./19.data')
scanners[0].set_offset((0,0,0))
scanners[0].set_mapping((1,2,3))
check_overlap(0)
for scanner in scanners:
  print(scanner.offset, scanner.mapping)

all_points = {}
for scanner in scanners:
  positions = scanner.get_global_positions()
  for pos in positions:
    all_points[pos] = True

# print(all_points)
print(f'count all points: {len(all_points)}')

max_distance = 0
for sa in scanners:
  for sb in scanners:
    dist = vector.manhatten_distance(sa.offset, sb.offset)
    max_distance = max(dist, max_distance)

print(f'max distance: {max_distance}')

# print(points2)

 # scanner 1    68, -1246,  -43 
 # scanner 2  1105, -1205, 1229 
 # scanner 3   -92, -2380,  -20
 # scanner 4   -20, -1133, 1061


relative = """
 0:  (  0,     0,     0) ( 1,  2,  3)
0-1: ( 68, -1246,   -43) (-1,  2, -3)
4-2: (168, -1125,    72) ( 2,  1, -3)
1-3: (160, -1134,   -23) ( 1,  2,  3)
1-4: ( 88,   113, -1104) ( 2, -3, -1)
"""