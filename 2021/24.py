
import util

def read_data():
  filename = './24.data'
  lines = util.read_data(filename)
  return lines

class ALU:
  def __init__(self, program):
    self.w = 0
    self.x = 0
    self.y = 0
    self.z = 0
    self.program = program

  def run(self, number):
    self.digits = list(number)
    for line in self.program:
      self.exec(line)

    # valid, if z is 0
    z_val = self.get_register("z")
    return z_val == 0

  def exec(self, line):
    tok = line.split(" ")
    op = tok[0]
    v1 = tok[1]
    v2 = None
    if len(tok) == 3:
      v2 = tok[2]
      v2 = self.evaluate(v2)
    if op == "inp":
      self.inp(v1)
    elif op == "add":
      self.add(v1, v2)
    elif op == "mul":
      self.mul(v1, v2)
    elif op == "div":
      self.div(v1, v2)
    elif op == "mod":
      self.mod(v1, v2)
    elif op == "eql":
      self.eql(v1, v2)
    else:
      raise NameError(f'bad op code:{op}')


  def inp(self, reg):
    val = self.digits.pop(0)
    self.set_register(reg, int(val))

  def add(self, reg, b):
    a = self.get_register(reg)
    self.set_register(reg, a + b)

  def mul(self, reg, b):
    a = self.get_register(reg)
    self.set_register(reg, a * b)

  def div(self, reg, b):
    a = self.get_register(reg)
    self.set_register(reg, int(a / b))

  def mod(self, reg, b):
    a = self.get_register(reg)
    self.set_register(reg, a % b)

  def eql(self, reg, b):
    a = self.get_register(reg)
    if a == b:
      self.set_register(reg, 1)
    else:
      self.set_register(reg, 0)

  def evaluate(self, v):
    # lstip("-") .  -3 => 3
    if v.lstrip("-").isdigit():
      return int(v)
    else:
      return self.get_register(v)

  def get_register(self, reg):
    if reg == 'w':
      return self.w
    elif reg == 'x':
      return self.x
    elif reg == 'y':
      return self.y
    elif reg == 'z':
      return self.z
    else:
      raise NameError(f'bad register:{reg}')


  def set_register(self, reg, val):
    if reg == 'w':
      self.w = val
    elif reg == 'x':
      self.x  = val
    elif reg == 'y':
      self.y = val
    elif reg == 'z':
      self.z = val
    else:
      raise NameError(f'bad register:{reg}')


program = read_data()
alu = ALU(program)
number = "13579246899999"
ok = alu.run(number)
print(f'{number} result:{ok} {alu.z}')