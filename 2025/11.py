# day 11 / advent of code 2025

with open("11.data") as f:
  lines = f.readlines()
lines = [line.strip("\n") for line in lines]


class Device:
  name = ""
  outputs = []

  def __init__(self):
    self.name = ""
    self.outputs = []

  def __str__(self):
    return f"Device(name={self.name}, outputs={self.outputs})"
  
  def __repr__(self):
    return f"Device(name={self.name}, outputs={self.outputs})"
  
  def parse_line(self, line):
    name, rest = line.split(": ")
    outputs = rest.split(" ")

    self.name = name
    self.outputs = [output.strip() for output in outputs]


devices = {}
for line in lines:
  device = Device()
  device.parse_line(line)
  devices[device.name] = device.outputs
# print(devices)

START = "you"
END = "out"


paths = []
path = [START]
paths = [path]
cont = True
while cont:
  cont = False
  new_paths = []
  for path in paths:
    name = path[-1]
    if name != END:
      cont = True
      outputs = devices[name]
      for output in outputs:
        new_path = path.copy()
        new_path.append(output)
        new_paths.append(new_path)
    else:
      new_paths.append(path)
  paths = new_paths

print("Part1 count of paths:", len(paths))