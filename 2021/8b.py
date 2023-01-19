
# https://adventofcode.com/2021/day/8

import util


def read_data():
  lines = util.read_data('./8.data')
  result = []
  for line in lines:
    tok = line.split("|")
    patterns = tok[0].split()
    patterns = [''.join(sorted(a)) for a in patterns]
    output = tok[1].split()
    output = [''.join(sorted(a)) for a in output]
    result.append( (patterns, output) )
  return result

def delta(a, b):
  result = []
  for x in a:
    if not x in b:
      result.append(x)
  return result

def contains(a, b):
  for x in b:
    if not x in a:
      return False
  return True

def segment_only_once(p1, p2):
  result = p1
  for w in p2:
    if w in result:
      result = result.replace(w, '')
    else:
      result = result + w
  return ''.join(sorted(result))

def count_duplicate(p1, p2):
  count = 0
  for c in p1:
    if c in p2:
      count = count +1
  return count

def decode(signals):
  signal_to_nr = {}
  pats_5 = []
  pats_6 = []
  for signal in signals:
    length = len(signal)
    if length == 2:
      pat_2 = signal
    if length == 3:
      pat_3 = signal
    if length == 4:
      pat_4 = signal
    if length == 7:
      pat_7 = signal
    if length == 5:
      pats_5.append(signal)
    if length == 6:
      pats_6.append(signal)

  sig_1 = pat_2
  sig_7 = pat_3
  sig_4 = pat_4
  sig_8 = pat_7

  # 7 has one segment more that 1
  wire_a = delta(pat_3, pat_2)[0]   

  # 3 is the only on of pats_5 fully contains 7
  for pat in pats_5:
    if contains(pat, sig_7):
      sig_3 = pat
      pats_5.remove(pat)
      break

  # pat_6 (0,6,9)  6 does not fully contains 1
  for pat in pats_6:
    if not contains(pat, sig_1):
      sig_6 = pat
      pats_6.remove(pat)
      break

  # all single segments of pats_5 fully fits into 0
  pat_25_single = segment_only_once(pats_5[0], pats_5[1])
  for pat in pats_6:
    if contains(pat, pat_25_single):
      sig_0 = pat
      pats_6.remove(pat)
      sig_9 = pats_6[0]
      pats_6.remove(sig_9)
      break

  # 5 and 4 results 3 duplicate segments
  # 2 and 4 results only 2 duplicate segments
  if count_duplicate(pats_5[0], sig_4) == 3:
    sig_5 = pats_5[0]
    sig_2 = pats_5[1]
  else:
    sig_2 = pats_5[0]
    sig_5 = pats_5[1]


  # print(pat_25_single)

  print(f'{pat_2} {pat_3} {pat_4} {pats_5} {pats_6}')

  signal_to_nr[sig_0] = 0
  signal_to_nr[sig_2] = 2
  signal_to_nr[sig_1] = 1
  signal_to_nr[sig_3] = 3
  signal_to_nr[sig_4] = 4
  signal_to_nr[sig_5] = 5
  signal_to_nr[sig_6] = 6
  signal_to_nr[sig_7] = 7
  signal_to_nr[sig_8] = 8
  signal_to_nr[sig_9] = 9
  return signal_to_nr

def translate(signals, signal_to_nr):
  result = []
  for signal in signals:
    signal = ''.join(sorted(signal))
    nr = signal_to_nr.get(signal)
    result.append(str(nr))
  str_result = ''.join(result)
  result = int(str_result)
  return result
  

def get_segment_length(nr):
  lenghts = { 0:6, 1:2, 2:5, 3:4, 4:4, 5:5, 6:6, 7:3, 8:7, 9:6 }
  return lenghts.get(nr)



data = read_data()
result_sum = 0
for dt in data:
  signal_to_nr = decode(dt[0])
  output = translate(dt[1], signal_to_nr)
  result_sum = result_sum + output
  print(f'{dt[0]}\n{output}')

print(f'Result: {result_sum}')



# print(data)
