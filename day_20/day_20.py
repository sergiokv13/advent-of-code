import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy

# Will return all the borders of a tile 
def get_tile_borders(m):
  return set([ ('').join(m[0]),  ('').join(m[-1]),  ('').join([r[0] for r in m]),  ('').join([r[-1] for r in m])])

# Will get an array of sets. Each element in the array are the borders of a valid tile option (rot90, rop180, flip, etc)
def get_tile_options(m):
  options = [get_tile_borders(m), get_tile_borders(np.flip(m))]
  for i in range(0,3):
    m = np.rot90(m)
    options = options + [get_tile_borders(m), get_tile_borders(np.flip(m))]

  return options

# Will run prev method for all tiles. This is in order to manage that data structure as the main one
def get_tiles_options(tiles):
  tiles_borders = {}
  for key, tile in tiles.items():
    tiles_borders[key] = get_tile_options(tile)

  return tiles_borders

# Get all corners of an image. We will iterate over each tile option vs all tiles options to find coincidences
# If the max # of coincidences is 2, then it means it has to be a border.
# will return the ids of the corner tiles
def get_image_corners(tiles_options):
  corners = []
  for key1, tile in tiles_options.items():
    max_coincidences = 0
    # iterate over 8 tile options
    for opt in tile:
      coincidences = 0
      for key2, tile2 in tiles_options.items():
        coincidence = False
        if key1 != key2:
          # iterate over 8 tile2 options
          for opt2 in tile2:
            if len(opt - opt2) < 4:
              coincidence = True
              break
        if coincidence : coincidences += 1
  
    max_coincidences = max([max_coincidences, coincidences])
    if max_coincidences == 2 : corners.append(key1)

  return corners


def print_matrix(m):
  for row in m:
    print(row)
  print()

tiles = {}
current_tile = []
current_matrix = []
for line in fileinput.input():
  clean_line = line.strip()

  if 'Tile' in clean_line:
    tile_str = clean_line.replace('Tile ', '')[:-1]
    if current_tile: tiles[current_tile] = np.array(current_matrix)
    current_matrix = []
    current_tile = tile_str
  elif clean_line:
    current_matrix.append(list(clean_line))
if current_tile: tiles[current_tile] = np.array(current_matrix)

tiles_options = get_tiles_options(tiles)
corners = get_image_corners(tiles_options)

# First Star
corners = get_image_corners(tiles_options)
print("corners: ", corners)
print(functools.reduce((lambda x, y: x * y), [int(el) for el in corners]))