import util

class Monkey:
  def __init__(self):
    self.items = []
    self.test_div_by = 1
    self.operation = ""
    self.test_true_throw_to = 0
    self.test_false_throw_to = 0
    self.inspect_counter = 0

  def __repr__(self):
    return f'start items:{self.items} operation:{self.operation} div:{self.test_div_by} {self.test_true_throw_to}/{self.test_false_throw_to}'

  def add_item(self, item):
    self.items.append(item)

  # -> [ (idx, wlevel), ]
  def throw_items(self):
    results = []
    for item in self.items:
      self.inspect_counter += 1
      wlevel = self.do_operation(item)

      # part-1 DIVIDED_BY = 3
      # part-2 DIVIDED_BY = 1
      if DIVIDED_BY != 1:
        wlevel = int(wlevel / DIVIDED_BY)
      rest = wlevel % self.test_div_by
      if rest == 0:
        results.append((self.test_true_throw_to, wlevel))
      else:
        results.append((self.test_false_throw_to, wlevel))
    self.items = []        
    return results

  def do_operation(self, nr):
    op_tok = self.operation.split("=")
    op_tok = op_tok[1].split()
    other_nr = op_tok[2]
    if other_nr == "old":
      other_nr = nr
    else:
      other_nr = int(other_nr)
    op = op_tok[1]
    if op == "+":
      return nr + other_nr
    if op == "*":
      return nr * other_nr
    raise Exception(f'bad operation:{self.operation}')


def get_data():
  monkeys = []
  lines = util.read_lines('./11.data')
  for line in lines:
    tok = line.split(":")
    cmd = tok[0]
    if cmd.startswith("Monkey"):
      monkeys.append(Monkey())
    elif cmd == "Starting items":
      tok_items = tok[1].split(",")
      items = [int(i) for i in tok_items]
      monkeys[-1].items = items
    elif cmd == "Operation":
      monkeys[-1].operation = tok[1]
    elif cmd == "Test":
      tok_test = tok[1].split()
      monkeys[-1].test_div_by = int(tok_test[2])
    elif cmd == "If true":
      tok_if = tok[1].split()
      monkeys[-1].test_true_throw_to = int(tok_if[3])
    elif cmd == "If false":
      tok_if = tok[1].split()
      monkeys[-1].test_false_throw_to = int(tok_if[3])
  return monkeys


def play_round(mod_item_by = 0):
  for monkey in monkeys:
    throw_items = monkey.throw_items()
    for throw_item in throw_items:
      (idx, item) = throw_item
      if mod_item_by > 0:
        item = item % mod_item_by
      monkeys[idx].add_item(item)


DIVIDED_BY = 3
monkeys = get_data()
for i in range(20):
  play_round()

inspects = sorted([monkey.inspect_counter for monkey in monkeys], reverse=True)
monkey_business = inspects[0] * inspects[1]

print(f'part-1: inspects:{inspects} monkey_business:{monkey_business}')

print("-------")

DIVIDED_BY = 1
prod_divs = 1
for monkey in monkeys:
  prod_divs *= monkey.test_div_by

monkeys = get_data()
for i in range(10000):
  play_round(prod_divs)

inspects = [monkey.inspect_counter for monkey in monkeys]
sorted_inspects = sorted([monkey.inspect_counter for monkey in monkeys], reverse=True)
monkey_business = sorted_inspects[0] * sorted_inspects[1]

print(f'part-2: inspects:{inspects} monkey_business:{monkey_business}')
