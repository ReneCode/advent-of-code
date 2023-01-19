import util

def get_data():
  lines = util.read_lines('./8.data')
  result = []
  for line in lines:
    row = [int(t) for t in line]
    result.append(row)
  return result


def visible(r, c, delta, trees):
  cnt_rows = len(trees)
  cnt_columns = len(trees[0])
  tree = trees[r][c]
  while True:
    r += delta[0]
    c += delta[1]
    if r < 0 or c < 0 or r >= cnt_rows or c >= cnt_columns:
      return True
    if trees[r][c] >= tree:
      return False

def view_distance(r, c, delta, trees):
  cnt_rows = len(trees)
  cnt_columns = len(trees[0])
  tree = trees[r][c]
  distance = 1
  while True:
    r += delta[0]
    c += delta[1]
    if r <= 0 or c <= 0 or r >= (cnt_rows-1) or c >= (cnt_columns-1):
      return distance
    if trees[r][c] >= tree:
      return distance
    else:
      distance += 1



trees = get_data()
# print(trees)
cnt_rows = len(trees)
cnt_columns = len(trees[0])
# the edge-trees are visible
total_visible = 2 * cnt_rows + 2 * cnt_rows - 4
# iterate over the trees inside the area
for row_idx in range( 1 , cnt_rows-1):
  for col_idx in range(1, cnt_columns-1):
    tree = trees[row_idx][col_idx]
    if ( visible(row_idx, col_idx, ( 0, 1), trees) or
         visible(row_idx, col_idx, ( 0,-1), trees) or
         visible(row_idx, col_idx, ( 1, 0), trees) or
         visible(row_idx, col_idx, (-1, 0), trees) ):
      # print (f'{row_idx},{col_idx}: {tree}')
      total_visible += 1

print(f'visible trees: {total_visible}')


max_score = 0

# iterate over the trees inside the area
for r in range( 1 , cnt_rows-1):
  for c in range(1, cnt_columns-1):
    dist_top =    view_distance(r, c, (-1, 0), trees)
    dist_bottom = view_distance(r, c, ( 1, 0), trees)
    dist_left =   view_distance(r, c, ( 0,-1), trees)
    dist_right =  view_distance(r, c, ( 0, 1), trees)
    score = dist_top * dist_bottom * dist_left * dist_right
    # print(f'{r},{c}: {dist_top},{dist_bottom},{dist_left},{dist_right}')
    max_score = max(max_score, score)

print(f'max score: {max_score}')

