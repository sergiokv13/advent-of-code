import fileinput

bit_mat = []
line_idx = 0
numbers = []
# Dict with the number and an array of positions for that number. eg. 25: [(1,2), (2,3)]
boards = []
# Matrix that will track the marked state of the board
boards_marked = []
# Dict with tuple of position and number, for lookup purposes. eg. (1,2): 25
boards_inv = []

current_board = {}
current_board_inv = {}
current_marked = []


for line in fileinput.input():
  if line_idx == 0:
    numbers = [int(n) for n in line.strip().split(',')]
  else:
    if line == '\n':
      if (len(current_board)): 
        boards.append(current_board)
        boards_marked.append(current_marked)
        boards_inv.append(current_board_inv)

      current_board = {}
      current_board_inv = {}
      current_marked = []
    else:
      current_marked_row = []
      for n in [int(n) for n in line.strip().split()]:
        current_marked_row.append(False)
        if (current_board.get(n)):
          current_board[n].append((len(current_marked), len(current_marked_row) - 1))
        else:
          current_board[n] = []
          current_board[n].append((len(current_marked), len(current_marked_row) - 1))
        current_board_inv[(len(current_marked), len(current_marked_row) - 1)] = n

      current_marked.append(current_marked_row)
  line_idx += 1
if (len(current_board)): 
  boards.append(current_board)
  boards_marked.append(current_marked)
  boards_inv.append(current_board_inv)

def print_matrix(m):
  for row in m:
    print(row)
  print()

def marked_win(marked):
  for i in range(5):
     row = marked[i]
     col = [row[i] for row in marked]

     if (sum(row) == 5): return True
     if (sum(col) == 5): return True
  return False

def calculate_score(board, board_inv, marked, last_num):
  local_sum = 0
  for i in range(5):
    for j in range(5):
      if not marked[i][j]:
        local_sum += board_inv[(i,j)]

  return local_sum * last_num

def play_bingo():
  first = None
  winners = []
  for number in numbers:
    for idx, board in enumerate(boards):
      marked = boards_marked[idx]
      if board.get(number):
        for el in board.get(number):
          i, j = el
          marked[i][j] = True

      if idx not in winners and marked_win(marked):
        winners.append(idx)
        score = calculate_score(board, boards_inv[idx], marked, number)
        if not first:
          first = score
        
  return first, score

scores = play_bingo()

# First Star

print(scores[0])


# Second Star

print(scores[1])