import util


class File:
  def __init__(self, size, name):
    self.size = size
    self.name = name

  def __repr__(self):
    return f'File: {self.name}:{self.size}'

def is_directory(obj):
  return isinstance(obj, Directory)

class Directory:
  def __init__(self, name):
    self.name = name
    self.content = []

  def __repr__(self):
    return f'dir:{self.name}, content:{self.content}'

  def get_subdir(self, name):
    dirs = self.get_dirs()
    for dir in dirs:
      if dir.name == name:
        return dir
    raise Exception(f'subdir {name} not found below {self.name}')


  def get_dirs(self):
    return [fd for fd in self.content if is_directory(fd)]

  def get_size(self):
    total = 0
    for file_or_dir in self.content:
      if is_directory(file_or_dir):
        dir = file_or_dir
        total += dir.get_size()
      else:
        file = file_or_dir
        total += file.size
    return total

  def add_subdir(self, name):
    self.content.append(Directory(name))

  def add_file(self, size, name):
    self.content.append(File(size, name))

  def fill_dir_size(self, parent_name, dir_size):
    dirs = self.get_dirs()
    for dir in dirs:
      # create full-name as key
      name = parent_name + "/" + dir.name
      size = dir.get_size()
      dir_size[name] = size
      dir.fill_dir_size(name, dir_size)

def read_data():
  lines = util.read_lines('./7.data')
  return lines

lines = read_data()
stack_dir = []
stack_dir.append(Directory('/'))

show_list = False
for line in lines:
  current_dir = stack_dir[-1]
  tok = line.split(" ")
  if show_list:
    if tok[0] == "$":
      show_list = False
    elif tok[0] == "dir":
      current_dir.add_subdir(tok[1])
    else:
      current_dir.add_file(int(tok[0]), tok[1])
  if not show_list and tok[0] == "$":
    cmd = tok[1]
    if cmd == "cd":
      name = tok[2]
      if name == "/":
        stack_dir = [stack_dir[0]]
      elif name == "..":
        stack_dir.pop()
      else:
        next_dir = current_dir.get_subdir(name)
        stack_dir.append(next_dir)
    elif cmd == "ls":
      show_list = True


root = stack_dir[0]
dir_size = { "/": root.get_size() }
root.fill_dir_size("", dir_size)
# print(dir_size)
# print(f'len: {len(dir_size)}')

all_sizes = [dir_size[name] for name in dir_size.keys()]
valid_sizes = [s for s in all_sizes if s < 100000]
total = sum(valid_sizes)
print(f'total size: {total}')

total_available = 70000000
needed_unused =   30000000
root_size = root.get_size()
current_free = total_available - root_size
min_cleanup = (needed_unused - current_free)
# print(f'root size:{root_size} {current_free} {min_cleanup}')
sorted_sizes = sorted(all_sizes)
for size in sorted_sizes:
  if size >= min_cleanup:
    print(f'size of dir that will be removed: {size}')
    break