import util

def read_data():
  lines = util.read_lines('./10.data')
  return lines

def calc_signal_strength(cycles, reg_x):
  if (cycles - 20) % 40 == 0:
    signal_strength = cycles * reg_x
    # print(f'cycle: {cycles} x:{reg_x}, signal strength:{signal_strength}')
    return signal_strength
  return 0

total_signal_strength = 0
reg_x = 1
cycles = 0
instructions = read_data()
for instruction in instructions:
  # print(f'{instruction}')

  token = instruction.split()
  cmd = token[0]
  if cmd == 'noop':
    cycles = cycles +1
    total_signal_strength += calc_signal_strength(cycles, reg_x)
  elif cmd == 'addx':
    cycles = cycles +1
    total_signal_strength += calc_signal_strength(cycles, reg_x)
    cycles = cycles +1
    total_signal_strength += calc_signal_strength(cycles, reg_x)
    # add AT THE END of the cycle
    val = int(token[1])
    reg_x = reg_x + val
    # print(f'x:{reg_x} cycle:{cycles}')
  else:
    raise Exception(f'bad instruction: {instruction}')

print(f'total signal strength: {total_signal_strength}')

## part 2

PIXEL_LIT = '#'
PIXEL_DARK = '.'
# crt 6 rows, each 40 columns
crt = [[PIXEL_DARK]*40 for i in range(6)]

def set_pixel(cy, sprite_pos):
  row = (cy-1) // 40
  pos = (cy-1) % 40
  pixel = PIXEL_DARK
  if pos in sprite_pos:
    pixel = PIXEL_LIT
  crt[row][pos] = pixel

def get_sprite_position(x):
  return [x-1, x, x+1]

reg_x = 1
cycles = 0
sprite_position = get_sprite_position(reg_x)
instructions = read_data()
for instruction in instructions:
  # print(f'{instruction}')
  token = instruction.split()
  cmd = token[0]
  if cmd == 'noop':
    cycles = cycles +1
    set_pixel(cycles, sprite_position)
  elif cmd == 'addx':
    cycles = cycles +1
    set_pixel(cycles, sprite_position)

    cycles = cycles +1
    set_pixel(cycles, sprite_position)

    # add AT THE END of the cycle
    val = int(token[1])
    reg_x = reg_x + val
    # change of reg_x => update sprite_position
    sprite_position = get_sprite_position(reg_x)
  else:
    raise Exception(f'bad instruction: {instruction}')

for row in crt:
  line = "".join(row)
  print(line)


