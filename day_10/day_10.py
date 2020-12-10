import fileinput
import functools
import re
import math
from collections import deque

def get_jolts_differences(nodes):
  count = {}; visited = set()
  q = deque(); q.append(0); visited.add(0)
  count[1] = 0; count[2] = 0; count[3] = 1

  while(q):
    top = q.popleft()
    for i in range(1,4):
      if (top + i) in nodes and (top + i) not in visited:
        q.append(top + i)
        visited.add(top + i)
        count[i] += 1
        break

  return count


count_memo = {}
def count_arrangements(nodes, idx = 0):
  if idx in count_memo:
    return count_memo[idx]

  if idx == len(nodes) - 1:
    return 1
  
  count = 0
  for idx2 in range(idx + 1, len(nodes)):
    if nodes[idx2] - nodes[idx] <= 3:
      count += count_arrangements(nodes, idx2)
  
  count_memo[idx] = count
  return count_memo[idx]


nodes = set()
for line in fileinput.input():
  nodes.add(int(line.strip()))

# First Star
jolt_diff = get_jolts_differences(nodes)
# print(jolt_diff[1] * jolt_diff[3])
# Second Star
nodes_arr = list(nodes)
nodes_arr.sort()
nodes_arr =  [0] + nodes_arr + [nodes_arr[-1] + 3]
print(count_arrangements(nodes_arr))