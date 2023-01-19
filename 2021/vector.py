
# vector
# (x,y,z)

# return p1 - p2
def sub(p1, p2):
  return (p1[0]-p2[0], p1[1]-p2[1], p1[2]-p2[2])

# return p1 + p2
def add(p1, p2):
  return (p1[0]+p2[0], p1[1]+p2[1], p1[2]+p2[2])


def manhatten_distance(v1, v2):
  (dx,dy,dz)= sub(v1, v2)
  return abs(dx) + abs(dy) + abs(dz)

class Matrix:
  #
  # (a, d, g, j)
  # (b, e, h, k)
  # (c, f, i, l)
  # (0, 0, 0, 1)
  def __init__(self,a,b,c,d,e,f,g,h,i,j,k,l):
    self.a = a
    self.b = b
    self.c = c
    self.d = d
    self.e = e
    self.f = f
    self.g = g
    self.h = h
    self.i = i
    self.j = j
    self.k = k
    self.l = l

  def translate(dx,dy,dz):
    return Matrix(0,0,0,0,0,0,0,0,0,dx,dy,dz)

  def multiply(max):
    pass