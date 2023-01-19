import util



def get_data():
    lines = util.read_lines('./12.data')
    start = None
    end = None
    rows = len(lines)
    cols = len(lines[0])
    ord_a = ord("a")
    board = []
    for r in range(rows):
        board_row = []
        for c in range(cols):
            field = lines[r][c]
            pos = (r,c)
            if field == "S":
                start = pos
                field = "a"
            elif field == "E":
                end = pos
                field = "z"
            val = ord(field) - ord_a + 1
            board_row.append(val)
        board.append(board_row)
    return (start, end, board)


# elevation (hight) a,b,c ... z
# start S (has elevation a)
# best signal E (has elevation z)

# minimal steps to go from S to E
# next field can only be one higher than the current field

class TableItem:
    def __init__(self, pos, height):
        self.pos = pos
        self.height = height
        self.steps = None
        self.prev_pos = None

    def __repr__(self):
        return f'{self.pos} cost:{self.steps} height:{self.height} prev:{self.prev_pos}'

    def set_steps(self, steps, prev_pos):
        if self.steps == None or self.steps > steps:
            self.steps = steps
            self.prev_pos = prev_pos
        else:
            raise Exception(f'do not set item to higher steps on pos:{self.pos}')

# https://www.programiz.com/dsa/dijkstra-algorithm
"""
function dijkstra(G, S)
    for each vertex V in G
        distance[V] <- infinite
        previous[V] <- NULL
        If V != S, add V to Priority Queue Q
    distance[S] <- 0
	
    while Q IS NOT EMPTY
        U <- Extract MIN from Q
        for each unvisited neighbour V of U
            tempDistance <- distance[U] + edge_weight(U, V)
            if tempDistance < distance[V]
                distance[V] <- tempDistance
                previous[V] <- U
    return distance[], previous[]
"""

class WayFinder:
    def __init__(self, board):
        self.board = board
        self.rows = len(board)
        self.cols = len(board[0])
        self.table = {}
        self.waiting_pos = []
        self.visited_positions = {}

    def init_table(self):
        for r in range(self.rows):
            for c in range(self.cols):
                pos = (r,c)
                height = self.board[r][c]
                item = TableItem(pos, height)
                self.table[pos] = item

    def add_waiting(self, pos):
        if not pos in self.waiting_pos:
            self.waiting_pos.append(pos)

    def remove_waiting(self, pos):
        self.waiting_pos.remove(pos)

    def get_min_waiting(self):
        # get item with min steps
        items = [self.get_item(pos) for pos in self.waiting_pos]
        min_steps = items[0].steps
        min_item = items[0]
        for item in items:
            if item.steps < min_steps:
                min_steps = item.steps
                min_item = item
        return min_item

    def is_visited(self, pos):
        return self.visited_positions.get(pos) != None

    def add_visited(self, pos):
        self.visited_positions[pos] = True



    def get_next_positions(self, pos, height):
        # next pos has to have next_height <= height+1
        (r,c) = pos
        test_positions = []
        if r > 0:
            test_positions.append( (r-1,c) )
        if (r+1) < self.rows:
            test_positions.append( (r+1,c) )
        if c > 0:
            test_positions.append( (r,c-1) )
        if (c+1) < self.cols:
            test_positions.append( (r,c+1) )

        next_positions = []
        for test_pos in test_positions:
            item = self.get_item(test_pos)
            if item.height <= (height+1):
                next_positions.append(test_pos)
        return next_positions

    def get_item(self, pos):
        return self.table[pos]

    def calc(self, start_pos):
        self.add_waiting(start_pos)
        item = self.get_item(start_pos)
        item.steps = 0

        while len(self.waiting_pos) > 0:
            current_item = self.get_min_waiting()
            next_positions = self.get_next_positions(current_item.pos, current_item.height)
            for next_pos in next_positions:
                if not self.is_visited(next_pos):
                    steps = current_item.steps + 1
                    next_item = self.get_item(next_pos)
                    if next_item.steps == None or next_item.steps > steps:
                        next_item.set_steps(steps, current_item.pos)
                    self.add_waiting(next_pos)
            self.add_visited(current_item.pos)
            self.remove_waiting(current_item.pos)

    def get_steps(self, pos):
        item = self.get_item(pos)
        return item.steps



(start_pos, end_pos, board) = get_data()
way_finder = WayFinder(board)
way_finder.init_table()

way_finder.calc(start_pos)
steps = way_finder.get_steps(end_pos)
print(f'part-1: shortest way: {steps}')



def get_start_positions():
    start_positions = []
    for r in range(len(board)):
        for c in range(len(board[0])):
            if board[r][c] == 1:    # eq "a"
                start_positions.append((r,c))
    return start_positions

start_positions = get_start_positions()
min_steps = None
for pos in start_positions:
    way_finder = WayFinder(board)
    way_finder.init_table()
    way_finder.calc(pos)
    steps = way_finder.get_steps(end_pos)
    print(f'start at:{pos} steps:{steps}')
    if steps != None:
        if min_steps == None:
            min_steps = steps
        else:        
            min_steps = min(min_steps, steps)

print(f'part-2: shortest way: {min_steps}')
