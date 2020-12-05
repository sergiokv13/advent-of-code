import fileinput
import functools
import re
import math


def get_seat_pos(code, bl = 0, bt = 127, code_pos = 0):
  mid_l = (bt + bl) // 2
  mid_u = math.ceil((bt + bl) / 2)

  if (code[code_pos] == 'F' or code[code_pos] == 'L'):
    if (code_pos == len(code) - 1):
      return bl
    return get_seat_pos(code, bl, mid_l, code_pos + 1)
  else:
    if (code_pos == len(code) - 1):
      return bt
    return get_seat_pos(code, mid_u, bt, code_pos + 1)

def get_seat_id(code):
  return get_seat_pos(code[:7]) * 8 + get_seat_pos(code[7:], 0, 7)

def get_missing(ids):
  all_ids = set([x * 8 + y for x in range(0, 127) for y in range(7)])
  possible = set(all_ids) - set(ids)

  for my_id in possible:
    if (my_id + 1 in ids and my_id - 1 in ids):
      return my_id

passes_ids = []
for line in fileinput.input():
  passes_ids.append(get_seat_id(line.strip()))

# First Star
print(max(passes_ids))
# Second Star
print(get_missing(passes_ids))
