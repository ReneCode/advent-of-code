import util
from itertools import combinations

time_left = 30

time_to_move_room = 1
time_to_close_valve = 1

START_NAME  = "AA"


# total_pressure = time * rate

class Valve:
  def __init__(self, name, rate, other_valves):
    self.name = name
    self.rate = rate
    self.other_valves = other_valves


def get_data():
  valves = []
  lines = util.read_lines("./16.data")
  for line in lines:
    tok = line.split(" ")
    name = tok[1]
    rate = int(tok[4].split("=")[1].split(";")[0])
    others = []
    idx = 9
    while idx < len(tok):
      other = tok[idx].split(",")[0]
      others.append(other)
      idx += 1
    valve = Valve(name, rate, others)
    valves.append(valve)
  return valves

class Path:
  def __init__(self, visited_names, target_names, time_left, pressure):
    self.visited_names = visited_names
    self.target_names = target_names
    self.pressure = pressure
    self.time_left = time_left
    self.finshed = False

  def get_current_name(self):
    return self.visited_names[-1]


class MinutePath:
  def __init__(self, current_name, current_target_name, target_names, pressure, moving_time):
    self.current_name = current_name
    self.current_target_name = current_target_name
    self.target_names = target_names
    self.pressure = pressure
    self.moving_time = moving_time
    self.visited = []


class WayFinder:
  def __init__(self, valves):
    self.valves = valves

  def get_valve(self, name):
    for valve in self.valves:
      if valve.name == name:
        return valve
    raise Exception(f'valve {name} not found')


  def calc_distance(self, v1_name, v2_name):
    # Dijstra
    que_names = [v1_name]
    distances = {v1_name:0}
    visited = set()
    while len(que_names) > 0:
      # get name from que with min distance
      cur_name = None
      min_dist = 999
      for name in que_names:
        if distances.get(name) != None and distances[name] < min_dist:
          min_dist = distances[name]
          cur_name = name
      
      cur = self.get_valve(cur_name)
      cur_distance = distances[cur_name]
      next_names = cur.other_valves
      for next_name in next_names:
        if not next_name in visited:
          if distances.get(next_name) == None  or distances[next_name] > (cur_distance+1):
            new_dist = cur_distance + 1
            if next_name == v2_name:
              return new_dist
            distances[next_name] = new_dist
            que_names.append(next_name)

      # remove from que
      que_names = [name for name in que_names if name != cur_name]
      visited.add(cur_name)
    return None

  def get_new_graph(self):
    rooms = [v for v in self.valves if v.rate > 0]

    # add first value
    start_valve = self.get_valve(START_NAME)
    rooms.append(start_valve)

    result = {}
    for idx_start in range(0, len(rooms)):
      start_name = rooms[idx_start].name
      distances = {}
      for idx_end in range(0, len(rooms)):
        if idx_start == idx_end:
          continue
        end_name = rooms[idx_end].name
        dist = self.calc_distance(start_name, end_name)
        distances[end_name] = dist
      result[start_name] = distances
    return result

  # algorithm that sums up all pressures on visited nodes
  # time is 'jumping' with the time to go from current node to target node
  def calc_all_single_path(self, start_time_left):
    graph = self.get_new_graph()

    first_name = START_NAME
    target_names = [i for i in graph if i != first_name]

    path = Path([first_name], target_names, start_time_left, 0)
    all_path = [path]
    for path in all_path:
      if path.time_left <= 0:
        continue
      
      current_name = path.get_current_name()
      for target_name in path.target_names:
        rate = self.get_valve(target_name).rate
        way_cost = graph[current_name][target_name]
        pressure = (path.time_left - way_cost -1) * rate
        new_time_left = path.time_left - way_cost -1
        new_target_names = [name for name in path.target_names if name != target_name]
        new_visited_names = path.visited_names + [target_name]
        new_pressure = path.pressure + pressure 
        new_path = Path(new_visited_names, new_target_names, new_time_left, new_pressure)
        all_path.append(new_path)
  
    return all_path

  # this algorithm is "ticked" on each minute
  # so on each minute we have to decide what to do (or just continue to walk to the target)
  def calc_all_minute_path(self, start_name, start_time_left, already_visited):
    graph = self.get_new_graph()
    start_target_names = [i for i in graph if i != start_name]
    start_target_names = [i for i in start_target_names if not i in already_visited]
    path = MinutePath(None, start_name, start_target_names, 0, 0)
    all_path = [path]
    minutes_left = start_time_left
    while minutes_left > 0:
      next_all_path = []
      for path in all_path:
        if len(path.target_names) == 0:
          next_all_path.append(path)
        elif path.moving_time > 0:
          # still moving
          path.moving_time -= 1
          next_all_path.append(path)

        elif path.moving_time == 0:
          # target reached
          target_names = path.target_names
          current_name = path.current_target_name
          if current_name in path.visited:
            raise Exception('ups duplicate visit', current_name)
          path.visited = path.visited + [current_name]
          if current_name in target_names:
            # remove it from tagets
            target_names = [n for n in target_names if n != current_name]
            # calc pressure
            rate = self.get_valve(current_name).rate
            pressure = minutes_left * rate
            path.pressure += pressure
          
          if len(target_names) == 0:
            path.target_names = []
            next_all_path.append(path)

          for new_target_name in target_names:
            if new_target_name in path.visited:
              raise Exception(f'ups do not target already visited node {new_target_name}')
            moving_time = graph[current_name][new_target_name]
            # remaining_target_names = [n for n in target_names if n != new_target_name]
            new_path = MinutePath(current_name, new_target_name, target_names, path.pressure, moving_time)
            new_path.visited = path.visited
            next_all_path.append(new_path)
        else:
          raise Exception('bad minutes')
      all_path = next_all_path
      minutes_left -= 1
    return all_path

  def get_best_path(self, start_name, time_left, visited):
    all_path = self.calc_all_minute_path(start_name, time_left, visited)
    all_path = sorted(all_path, reverse=True, key=lambda p: p.pressure)        
    best_path = all_path[0]
    return best_path

def part_1():
  valves = get_data()
  way_finder = WayFinder(valves)
  all_path = way_finder.calc_all_single_path(30)
  pressures = sorted([p.pressure for p in all_path], reverse=True)        
  pressure = pressures[0]
  print(f'part-1 len: {len(all_path)} => pressure:{pressure}')

def part_2():
  valves = get_data()
  way_finder = WayFinder(valves)

  graph = way_finder.get_new_graph()
  all_names = [name for name in graph if name != START_NAME]
  max_pressure = 0
  best_way = None
  for comb_count in range(1, (len(all_names)//2)+1):
    all_combinations = combinations(all_names, comb_count)
    for my_targets in all_combinations:
      ele_targets = [name for name in all_names if not name in my_targets]

      my_best_path = way_finder.get_best_path(START_NAME, 26, ele_targets)
      ele_best_path = way_finder.get_best_path(START_NAME, 26, my_targets)
      my_pressure = my_best_path.pressure
      ele_pressure = ele_best_path.pressure
      pressure = my_pressure + ele_pressure
      way = f'me:{my_targets} => {my_pressure} elephant:{ele_targets} => {ele_pressure} result:{pressure} (max:{max_pressure})'
      print(way)
      if pressure > max_pressure:
        max_pressure = pressure
        best_way = way

  print()
  print(best_way)


part_1()
part_2()