
t = __import__('22-2')

def test_insert_more_outside():
  result = t.get_left_cut_right((2,5), (6,8))
  assert result == None

def test_insert():
  result = t.get_left_cut_right((2,5), (3,4))
  assert result == ((2,2), (3,4),(5,5))
  
def test_insert_more_left():
  result = t.get_left_cut_right((2,5), (1,4))
  assert result == (None, (2,4),(5,5))

def test_insert_more_rightborder():
  result = t.get_left_cut_right((2,5), (5,7))
  assert result == ((2,4), (5,5),None)


def test_cutout_1():
  box_a = t.Box((10,30), (20,40), (30,50))
  box_b = t.Box((1,5), (20,25), (60,70))
  result = box_a.cutout(box_b)
  expect = [
    t.Box((10,30), (26,40), (30,50))
  ]
  assert result == expect

def test_cutout_2():
  box_a = t.Box((10,30), (20,40), (30,50))
  box_b = t.Box((15,20), (20,25), (60,70))
  result = box_a.cutout(box_b)
  expect = [
    t.Box((10,14), (20,40), (30,50)),
    t.Box((21,30), (20,40), (30,50)),
    t.Box((15,20), (26,40), (30,50))
  ]
  assert result == expect

def test_cutout_boxes_do_not_entersect():
  box_a = t.Box((10,30), (20,40), (30,50))
  box_b = t.Box((1,2), (2,5), (60,70))
  result = box_a.cutout(box_b)
  expect = [
    box_a
  ]
  assert result == expect



def test_cutout_3():
  box_a = t.Box((10,10), (12,12), (10,12))
  box_b = t.Box((10,10), (10,10), (10,10))
  result = box_a.cutout(box_b)
  expect = [
    t.Box((10,10), (12,12), (11,12))
  ]
  assert result == expect

def test_cutout_4():
  box_a = t.Box((10,10), (10,12), (10,12))
  box_b = t.Box((9,11), (9,11), (9,11))
  result = box_a.cutout(box_b)
  expect = [
    t.Box((10,10), (12,12), (11,12))
  ]
  # assert result == expect


def test_volume():
  box = t.Box((10,30), (20,30), (30,30))
  assert box.volume() == 21 * 11 * 1
