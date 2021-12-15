import fileinput
from typing import DefaultDict
import math
import numpy as np
import sys
np.set_printoptions(threshold=sys.maxsize)

# Todo, use heap or priority queue
def pop_min(queue, dist):
    min_dist = float("inf")
    min_idx = None
    for idx, node in enumerate(queue):
        if dist[node] < min_dist:
            min_dist = dist[node]
            min_idx = idx
    return queue.pop(min_idx)

def getp(matrix, i, j, default = None):
  try:
    if i < 0 or j < 0: return default
    return matrix[i][j]
  except:
    return default

def shortest_path(init, end, mat):
    # distance from init to any node
    dist = DefaultDict(lambda:float("inf"))
    dist[init] = 0

    queue = [init]
    visited = set()

    while(queue):
        node = pop_min(queue, dist)
        x,y = node
        visited.add(node)

        for neighbor in [(x+1,y), (x-1,y), (x,y+1), (x,y-1)]:
            val = getp(mat, neighbor[0], neighbor[1])
            if val and (neighbor not in visited):
                if dist[neighbor] > dist[node] + val:
                    dist[neighbor] = dist[node] + val
                    queue.append(neighbor)

    return dist[end]

def extend_mat(mat):
    np_mat = np.array(mat)
    new_mat = np_mat.copy()
    add_mat = np_mat.copy()

    # add on X
    for i in range(4):
        add_mat = add_mat + 1
        add_mat[add_mat > 9] = 1
        new_mat = np.hstack((new_mat, add_mat))

    add_mat = new_mat.copy()
    # add on Y
    for i in range(4):
        add_mat = add_mat + 1
        add_mat[add_mat > 9] = 1
        new_mat = np.vstack((new_mat, add_mat))

    return new_mat


mat = []
for line in fileinput.input():
    mat.append([int(el) for el in line.strip()])

# First Star

print(shortest_path((0,0), (len(mat)-1, len(mat)-1), mat))

# Second Star
new_mat = extend_mat(mat)
print(shortest_path((0,0), (len(new_mat)-1, len(new_mat)-1), new_mat))


