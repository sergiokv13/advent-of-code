import fileinput

mat_s1 = [['.' for _i in range(1000)] for _j in range(1000)]
mat_s2 = [['.' for _i in range(1000)] for _j in range(1000)]
overlap_count = 0

def draw_line(mat, init, end, diagonal = False):
  x_init, y_init = init
  x_end, y_end = end

  if (x_init == x_end or y_init == y_end):
    x_init, x_end = sorted([x_init, x_end])
    y_init, y_end = sorted([y_init, y_end])
    for x in range(x_init, x_end + 1):
      for y in range(y_init, y_end + 1):
        if mat[y][x] != '.': mat[y][x] += 1
        else: mat[y][x] = 1
  elif (diagonal):
    for idx in range(abs(x_end - x_init) + 1):
      y = y_init + idx if y_end > y_init else y_init - idx
      x = x_init + idx if x_end > x_init else x_init - idx
  
      if mat[y][x] != '.': mat[y][x] += 1
      else: mat[y][x] = 1

  return mat

def calculate_overlap(mat):
  overlap = 0
  for row in mat:
    for el in row:
      if el != '.' and el > 1: overlap += 1
  return overlap
   
def print_matrix(m):
  for row in m:
    print([str(el) for el in row])
  print()

for line in fileinput.input():
  init = [int(el) for el in list(line.split(" -> ")[0].split(','))]
  end = [int(el) for el in list(line.split(" -> ")[1].split(','))]

  draw_line(mat_s1, init, end)
  draw_line(mat_s2, init, end, True)
  # print(init, end)
  # print_matrix(mat_s2)

# First Star

print(calculate_overlap(mat_s1))

# Second Star

print(calculate_overlap(mat_s2))