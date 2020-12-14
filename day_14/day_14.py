import fileinput
import functools
import re
import math
import operator
from collections import deque

def get_mask_value(mask, val):
  binary = list(bin(val)[2:])
  diff = len(mask) - len(binary)
  binary = [0 for v in range(0, diff)] + binary
  res = []
  for idx in range(0, len(mask)):
    if (mask[idx] == 'X'):
      res.append(binary[idx])
    else:
      res.append(mask[idx])
  res = ''.join([str(elem) for elem in res])
  return int(res, 2)

def get_options(arr, res_arr = []):
  curr_sum = 0; has_x = False
  for idx2 in range(0, len(arr)):
    if (arr[idx2] == 'X'):
      has_x = True; arr[idx2] = '0'
      get_options(arr, res_arr)
      arr[idx2] = '1'
      get_options(arr, res_arr)
      arr[idx2] = 'X'
      return

  if not has_x:
    res = ''.join([str(elem) for elem in arr])
    bin_val = int(res,2)
    if bin_val not in visited:
      visited.add(bin_val)
      res_arr.append(bin_val)

  return curr_sum

def get_memory_addresses(mask, val):
  binary = list(bin(val)[2:])
  diff = len(mask) - len(binary)
  binary = [0 for v in range(0, diff)] + binary
  res = []

  for idx in range(0, len(mask)):
    if (mask[idx] == 'X'):
      res.append('X')
    elif mask[idx] == '0':
      res.append(binary[idx])
    else:
      res.append(mask[idx])
  mem_arr = []
  get_options(res, mem_arr)
  return mem_arr

mask = []
memo = {}
memo2 = {}
for line in fileinput.input():
  instruction, val = [s.strip() for s in line.strip().split('=')]
  if instruction == 'mask':
    mask = list(val)
  else:
    memo[instruction] = get_mask_value(mask, int(val))
    # This is for second star
    mem_val = int(instruction.replace('mem[','').replace(']', ''))
    visited = set()
    memory_addresses = get_memory_addresses(mask, mem_val)
    for m in memory_addresses:
      memo2[m] = int(val)


# First Star
print(sum(memo.values()))

# Second Star
print(sum(memo2.values()))