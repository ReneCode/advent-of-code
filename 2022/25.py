import util

NUM_MAPPING = {
  "2": 2,
  "1": 1,
  "0": 0,
  "-": -1,
  "=": -2
}


"""
5-system

            5
        5^2
    5^3
5^4  

125  25  5  1

example:
2 = - 0 1

2*625
  -2 * 125
    -1 * 25
      0 * 5
         1 * 1

1250
  -250
     -25
       0
         1

sum => 976

"""


MAP_DIC_TO_SNAFU = {
  0: "0",
  1: "1",
  2: "2",
  3: "=",  # -2
  4: "-"   # -1
}


def get_data():
  lines = util.read_lines("./25.data")
  return lines

def from_snafu(snafu):
  total = 0
  digits = snafu.strip()
  nr_len = len(digits)
  result = 0
  for i in range(nr_len):
    c = digits[nr_len -1 -i]
    fac = NUM_MAPPING[c]
    val = pow(5, i) * fac
    result += val
    # print(c, fac, val)
  return result

def to_snafu(nr):
  n = 0
  rest = nr
  last_val = 0
  result = []
  while True:
    n += 1
    rest = rest - (last_val * pow(5, n-2))
    if rest == 0:
      break
    val = (rest % pow(5, n)) / pow(5,n-1)
    # val is 0,1,2,3,4
    snafu_digit = MAP_DIC_TO_SNAFU[val]
    result.insert(0, snafu_digit)
    if val >= 3:
      val -= 5
    # val is -2,-1,0,1,2
    last_val = val

  return "".join(result)

lines = get_data()
total = 0
for snafu in lines:
  result = from_snafu(snafu)
  total += result

result_snafu = to_snafu(total)
print(f'part-1 result:{total} => {result_snafu}')