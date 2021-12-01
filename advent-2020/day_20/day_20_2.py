import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy

border_size = None

# Will return all the borders of a tile 
def get_tile_borders(m):
  return [('').join(m[0]), ('').join([r[-1] for r in m]), ('').join(m[-1]), ('').join([r[0] for r in m])]

# Will get an array of sets. Each element in the array are the borders of a valid tile option (rot90, rop180, flip, etc)
def get_tile_options(m):
  options = [m, np.flip(m,1)]
  for i in range(0,3):
    m = np.rot90(m)
    options = options + [m, np.flip(m,1)]
  return options

# Will run prev method for all tiles. This is in order to manage that data structure as the main one
def get_tiles_options(tiles):
  tiles_borders = {}
  for key, tile in tiles.items():
    tiles_borders[key] = get_tile_options(tile)
  return tiles_borders

def match_border(bs, direction, bs2):
  if (direction == 0 and bs == bs2[2]) : return True
  if (direction == 1 and bs == bs2[3]) : return True
  if (direction == 2 and bs == bs2[0]) : return True
  if (direction == 3 and bs == bs2[1]) : return True
  return False

def find_neighboors(node, position, tiles, visited):
  neighboors = []
  # We already know the position for first one
  for direction, border in enumerate(get_tile_borders(tiles[node][int(position)])):
    for possible in (set(tiles.keys()) - visited):
      if possible == node : continue
      found = False
      for idx, options_possible in enumerate(tiles[possible]):
        if match_border(''.join(border), direction, get_tile_borders(options_possible)) and not found:
          neighboors.append((f"{possible}-{idx}", direction))
          found = True
        if found : break
      if found : break
  return neighboors

# We know that there is only one match with the borders because the solution on star 1 worked haha
# Using any corner as our start point (We get that from star 1)
# Adding -(0-8) to the key which represent the square position

def build_square_graph(tiles, init_node):
  my_graph = {}
  queue = [init_node]
  visited = set([init_node.split('-')[0]])

  while(queue):
    complete_node = queue.pop(0)
    node, pos = complete_node.split('-')
    neighboors = find_neighboors(node, pos, tiles, visited)
    my_graph[complete_node] = neighboors

    for n,direction in neighboors:
      if n.split('-')[0] not in visited:
        visited.add(n.split('-')[0])
        queue = queue + [n]

  return my_graph

def build_square_matrix_keys(graph, node, m, row, col):
  m[row][col] = node
  for n, position in graph[node]:
    if (position == 0) : build_square_matrix_keys(graph, n, m, row-1, col)
    if (position == 1) : build_square_matrix_keys(graph, n, m, row, col+1)
    if (position == 2) : build_square_matrix_keys(graph, n, m, row+1, col)
    if (position == 3) : build_square_matrix_keys(graph, n, m, row, col-1)

  return m

def remove_nones(m):
  new_m = []
  for row in m:
    if not all(el is None for el in row):
      new_m.append([el for el in row if el])

  return new_m

def build_image(m, tiles):
  image = None
  for row in range(border_size):
    big_row = None
    for col in range(border_size):
      key, pos = m[row][col].split('-')
      tile_data = tiles[key][int(pos)]
      # remove borders
      tile_data = tile_data[1:-1]
      tile_data = [el[1:-1] for el in tile_data]
      if big_row is not None:
        # zeros = np.full(len(tile_data), '   ').reshape(len(tile_data),1)
        # tile_data = np.hstack((zeros, tile_data))
        big_row = np.hstack((big_row, tile_data))
      else:
        big_row = tile_data
    if image is not None:
      # zeros = np.full(len(big_row[0]), '   ').reshape(1,len(big_row[0]))
      # big_row = np.vstack((zeros, big_row))
      image = np.vstack((image, big_row))
    else:
      image = big_row
  return image

def g(i,r,c):
  val = None
  try:
    val = i[r][c]
  except IndexError:
    val = None
  return val

def is_sea_monster(i, r, c):
  r_1 = g(i,r-1,c+18)=='#'  
  r0 = g(i,r,c)=='#' and g(i,r,c+5)=='#' and g(i,r,c+6)=='#' and g(i,r,c+11)=='#' and g(i,r,c+12)=='#' and g(i,r,c+17)=='#' and g(i,r,c+18)=='#' and g(i,r,c+19)=='#'
  r1 = g(i,r+1,c+1)=='#' and g(i,r+1,c+4)=='#' and g(i,r+1,c+7)=='#' and g(i,r+1,c+10)=='#' and g(i,r+1,c+13)=='#' and g(i,r+1,c+16)=='#'
  return r_1 and r0 and r1

def replace_sea_monster(image, r, c):
  image[r-1,c+18]='O'
  image[r,c]='O' 
  image[r,c+5]='O' 
  image[r,c+6]='O' 
  image[r,c+11]='O' 
  image[r,c+12]='O' 
  image[r,c+17]='O' 
  image[r,c+18]='O' 
  image[r,c+19]='O'
  image[r+1,c+1]='O' 
  image[r+1,c+4]='O'
  image[r+1,c+7]='O'
  image[r+1,c+10]='O'
  image[r+1,c+13]='O'
  image[r+1,c+16]='O'
  return image

def count_not_sea_monster(image):
  # print_np_matrix(image)
  for row in range(len(image)):
    for col in range(len(image[0])):
      if is_sea_monster(image, row, col):
        replace_sea_monster(image, row, col)
        #print_np_matrix(image)
  count = 0
  for v in image:
    for rc in v:
      if rc == '#' : count += 1

  return count

def solve_2(image):
  min_val = float("inf")
  min_val = min([count_not_sea_monster(image), min_val])
  min_val = min([count_not_sea_monster(np.flip(image)), min_val])

  for i in range(0,3):
    image = np.rot90(image)
    min_val = min([count_not_sea_monster(image), min_val])
    min_val = min([count_not_sea_monster(np.flip(image)), min_val])

  return min_val

def print_matrix(m):
  for row in m:
    print(row)
  print()

def print_np_matrix(m):
  for row in m:
    print(row.tolist())
  print()

tiles = {}
current_tile = []
current_matrix = []
for line in fileinput.input():
  clean_line = line.strip()

  if 'Tile' in clean_line:
    tile_str = clean_line.replace('Tile ', '')[:-1]
    if current_tile : tiles[current_tile] = np.array(current_matrix)
    current_matrix = []
    current_tile = tile_str
  elif clean_line:
    current_matrix.append(list(clean_line))
if current_tile : tiles[current_tile] = np.array(current_matrix)

border_size = int(math.sqrt(len(tiles)))

#from prev Star
#corners:  ['2693', '1549', '3539', '3709']
# test corners:  ['1951', '1171', '2971', '3079']
init_node = '2693-0'

tiles_options = get_tiles_options(tiles)
graph = build_square_graph(tiles_options, init_node)
matrix = build_square_matrix_keys(
  graph,
  init_node,
  [[None for _ in range(border_size * 2)] for _ in  range(border_size * 2)],
  border_size,
  border_size,
)

matrix = remove_nones(matrix)
print_matrix(matrix)
image = build_image(matrix, tiles_options)
# print_np_matrix(image)
print(solve_2(image))
