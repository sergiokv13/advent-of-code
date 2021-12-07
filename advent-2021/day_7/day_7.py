import fileinput
import sys
sys.setrecursionlimit(10000)

positions = []
for line in fileinput.input():
  positions = [int(n) for n in line.split(',')]


def get_min_fuel_1(positions):
  min_pos = min(positions)
  max_pos = max(positions)

  min_fuel = float('inf')
  for target in range(min_pos, max_pos + 1):
    current_fuel = 0
    for pos in positions:
      current_fuel += abs(target-pos)
    
    if (current_fuel < min_fuel):
      min_fuel = current_fuel
  
  return min_fuel

steps_memo = {0: 0}
def get_steps_fuel(steps):
  if steps_memo.get(steps, None) is not None: return steps_memo.get(steps)
  steps_memo[steps] = get_steps_fuel(steps - 1) + steps
  return steps_memo[steps]


def get_min_fuel_2(positions):
  min_pos = min(positions)
  max_pos = max(positions)

  min_fuel = float('inf')
  for target in range(min_pos, max_pos + 1):
    current_fuel = 0
    for pos in positions:
      current_fuel += get_steps_fuel(abs(target-pos))
    
    if (current_fuel < min_fuel):
      min_fuel = current_fuel
  
  return min_fuel

# First Star

print(get_min_fuel_1(positions))

# # Second Star

print(get_min_fuel_2(positions))

