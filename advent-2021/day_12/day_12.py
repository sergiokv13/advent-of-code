import fileinput
from typing import DefaultDict

G = DefaultDict(list)

def count_valid_paths_to_end(current_node, visited_small = set()):
  if current_node == 'end': return 1
  if current_node != 'start' and current_node.islower() and current_node in visited_small: return 0
  else: visited_small.add(current_node)
  
  valid_paths = 0
  for neighboor in G[current_node]:
    valid_paths += count_valid_paths_to_end(neighboor, visited_small)
  
  if current_node in visited_small: visited_small.remove(current_node)

  return valid_paths

# We could add memo here, but solution is still fast enough to don't prioritize that
def count_valid_paths_to_end_2(current_node, visited_small = DefaultDict(int), visited_twice = False):
  if current_node == 'end': return 1
  # Visiting small cave
  if current_node != 'start' and current_node.islower():
    # If we already visited the node 2 times
    if visited_small[current_node] == 2: return 0
    # If we could visit the node for a second time, but we already visited twice
    if visited_small[current_node] == 1 and visited_twice: return 0
    visited_small[current_node] += 1
    # If we are visiting twice, then we change the flag
    if visited_small[current_node] == 2: visited_twice = True

  valid_paths = 0
  for neighboor in G[current_node]:
    valid_paths += count_valid_paths_to_end_2(neighboor, visited_small, visited_twice)
  
  if visited_small[current_node]: visited_small[current_node] -= 1

  return valid_paths

for line in fileinput.input():
  n1, n2 = line.strip().split('-')
  # Build graph
  if n1 == 'start': G[n1].append(n2)
  elif n1 == 'end': G[n2].append(n1)
  elif n2 == 'start': G[n2].append(n1)
  elif n2 == 'end': G[n1].append(n2)
  else:
    G[n1].append(n2)
    G[n2].append(n1)

# First Star

print(count_valid_paths_to_end('start'))

# Second Star

print(count_valid_paths_to_end_2('start'))

