import fileinput
import functools
import re
import math

# First Star
def run_until_repeated(instructions):
  counter = 0; idx = 0; visited = {}
  while(idx < len(instructions)):
    if idx in visited:
      return counter

    visited[idx] = True
    instruction, value = instructions[idx]
    if (instruction == 'acc'):
      counter += int(value)
      idx += 1
    if (instruction == 'jmp'):
      idx += int(value)
    if (instruction == 'nop'):
      idx += 1
  
  return counter

# Second Star
def run_until_finished(instructions):
  counter = 0; idx = 0; visited = {}
  while(idx < len(instructions)):
    if idx in visited:
      return False
    visited[idx] = True
    instruction, value = instructions[idx]
    if (instruction == 'acc'):
      counter += int(value)
      idx += 1
    if (instruction == 'jmp'):
      idx += int(value)
    if (instruction == 'nop'):
      idx += 1
  return counter

def find_counter_after_fix(instructions):
  for idx in range(len(instructions)):
    instruction, value = instructions[idx]
    new_instructions = instructions[:]
    counter = False
    if (instruction == 'jmp'):
      new_instructions[idx] = ('nop', value)
      counter = run_until_finished(new_instructions)
    if (instruction == 'nop'):
      new_instructions[idx] = ('jmp', value)
      counter = run_until_finished(new_instructions)
    if counter is not False:
      return counter

instructions = []
for line in fileinput.input():
  instruction = line[:3].strip()
  instruction_weight = line[3:].strip()
  instructions.append((instruction, instruction_weight))

# First Star
print(run_until_repeated(instructions))
# Second Star
print(find_counter_after_fix(instructions))