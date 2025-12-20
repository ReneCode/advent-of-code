import math

with open("8.data") as f:
  lines = f.readlines()
lines = [line.strip("\n") for line in lines]


def parse_point(line):
  x, y, z = line.split(",")
  return (int(x), int(y), int(z))


def distance(p1, p2):
  return math.sqrt(pow((p1[0] - p2[0]),2) + pow((p1[1] - p2[1]),2) + pow((p1[2] - p2[2]),2))
          
def get_distances(points):
  distances = []
  for i in range(len(points)):
    for j in range(i+1, len(points)):
      distances.append(((i,j), distance(points[i], points[j])))
  return list(sorted(distances, key=lambda x: x[1]))


def pair_will_connect_two_circuits(idx1, idx2, circuits):
  new_circuits = []
  circuit1 = None
  circuit2 = None
  for circuit in circuits:
    if idx1 in circuit:
      circuit1 = circuit
    if idx2 in circuit:
      circuit2 = circuit
  if circuit1 is not None and circuit2 is not None and circuit1 != circuit2:
    new_circuit = list(set(circuit1 + circuit2))
    for circuit in circuits:
      if circuit != circuit1 and circuit != circuit2:
        new_circuits.append(circuit)
    new_circuits.append(new_circuit)
    return new_circuits
  return None


def part1():
  points = [parse_point(line) for line in lines]
  distances  = get_distances(points)

  circuits = []
  for i_tuple, tuple in enumerate(distances):
    if i_tuple == 1000:
      # we are done
      break

    # print("Processing pair:", tuple[0])
    (idx1, idx2) = tuple[0]

    new_circuits = pair_will_connect_two_circuits(idx1, idx2, circuits)
    if new_circuits is not None:
      circuits = new_circuits
      continue

    for circuit in circuits:
      if idx1 in circuit and idx2 in circuit:
        # both points already in the same circuit
        continue

    circuit_merged = False
    for circuit in circuits:
      if idx1 in circuit or idx2 in circuit:
        if idx1 not in circuit:
          circuit.append(idx1)
        if idx2 not in circuit:
          circuit.append(idx2)
        circuit_merged = True
        break
  
    if not circuit_merged:
      circuits.append([idx1, idx2])
# print("circuits after processing:", circuits)


  # print("Final Circuits:", circuits)

  # take the three longest circuits and multiply their lengths
  circuits_length = [len(circuit) for circuit in circuits]
  cir_sorted = sorted(circuits_length)
  cir_orderd = list(reversed(cir_sorted))
  result = cir_orderd[0] * cir_orderd[1] * cir_orderd[2]
  print("Part1 Result:", result)


def all_points_in_one_circuit(circuits, total_points):
  if len(circuits) == 1:
    circuit = circuits[0]
    for point_idx in range(total_points):
      if point_idx not in circuit:
        return False
    return True
  else:
    return False

def part2():
  points = [parse_point(line) for line in lines]
  distances  = get_distances(points)

  circuits = []

  for tuple in distances:
    # print("part 2 processing...", tuple[0], len(circuits))


    len_points = len(points)
    # if all_points_in_one_circuit(circuits, len_points):
    if len(circuits) > 0 and len(circuits[0])==len_points:
      p1 = points[idx1]
      p2 = points[idx2]
      # print("Final Circuits:", circuits)
      # print("All points connected into one circuit.", tuple[0], p1, p2)
      print("Part2 Result:", p1[0] * p2[0])
      # we are done
      break

    # print("Processing pair:", tuple[0])
    (idx1, idx2) = tuple[0]

    new_circuits = pair_will_connect_two_circuits(idx1, idx2, circuits)
    if new_circuits is not None:
      circuits = new_circuits
      continue

    for circuit in circuits:
      if idx1 in circuit and idx2 in circuit:
        # both points already in the same circuit
        continue

    circuit_merged = False
    for circuit in circuits:
      if idx1 in circuit or idx2 in circuit:
        if idx1 not in circuit:
          circuit.append(idx1)
        if idx2 not in circuit:
          circuit.append(idx2)
        circuit_merged = True
        break
  
    if not circuit_merged:
      circuits.append([idx1, idx2])



part1()
part2()
