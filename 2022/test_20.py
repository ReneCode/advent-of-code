

def move(sequence, nr):
  seq_len = len(sequence)
  start_idx = sequence.index(nr)
  end_idx = (start_idx + nr + seq_len - 1) % (seq_len - 1)
  end_idx_2 = (start_idx + nr - 1) % (seq_len - 1)
  print(f"{end_idx} / {end_idx_2}")
  sequence.pop(start_idx)
  sequence.insert(end_idx,nr)
  return sequence
  


def __move(sequence, nr):
  seq_len = len(sequence)
  start_idx = sequence.index(nr)
  sequence[start_idx] = None
  end_idx = (start_idx + nr + seq_len - 1) % (seq_len -1)
  sequence.insert(end_idx,nr)
  sequence.remove(None)
  return sequence
  



def test_a():
  assert move([1,2,3,4],2) == [2,1,3,4]

def test_1():
  assert move([1,2,-3,3,-2,0,4], 1) == [2,1,-3,3,-2,0,4]

def test_2():
  assert move([2,1,-3,3,-2,0,4], 2) == [1,-3,2,3,-2,0,4]

def test_3():
  assert move([1,-3,2,3,-2,0,4], -3) == [1,2,3,-2,-3,0,4]

test_a()