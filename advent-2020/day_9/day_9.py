import fileinput
import functools
import re
import math

def is_valid(idx, xmas_numbers, preamble = 25):
  sum_set = set()
  val = xmas_numbers[idx]
  for n in xmas_numbers[idx-preamble:idx]:
    if (val - n) in sum_set:
      return True
    sum_set.add(n)

  return False

def contiguous_range(idx, xmas_numbers):
  expected = xmas_numbers[idx]
  current_sum = 0; idx_start = 0; idx_end = 0
  for n in xmas_numbers[0:idx]:
    while (current_sum > expected):
      current_sum -= xmas_numbers[idx_start]
      idx_start += 1

    if (current_sum == expected):
      return (xmas_numbers[idx_start:idx_end])

    current_sum += n 
    idx_end += 1

  return None


xmas_numbers = []
for line in fileinput.input():
  xmas_numbers.append(int(line.strip()))

# Preamble
preamble = 25
for idx in range(preamble, len(xmas_numbers)):
  if not is_valid(idx, xmas_numbers, preamble):
    # First Star
    print(xmas_numbers[idx])
    # Second Star
    c_range = contiguous_range(idx, xmas_numbers)
    print(max(c_range) + min(c_range))
    break
