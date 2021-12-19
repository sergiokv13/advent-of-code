import fileinput
from typing import DefaultDict
from itertools import permutations
import re
import math

MIN_BEACOUN_COUNT = 12

matrix = DefaultDict(int)

def build_by_cols(xc, yc, zc):
    return [(x, y, z) for x,y,z in zip(xc, yc, zc)]

def change_sign(col):
    return [(el * -1) for el in col]

def get_variants(locations):
    variants = []
    x_col = [loc[0] for loc in locations]
    y_col = [loc[1] for loc in locations]
    z_col = [loc[2] for loc in locations]
    count = 0
    # todo remove repeated variants (*2)
    for x,y,z in list(permutations([x_col,y_col,z_col])):
        for changed_x in [x, change_sign(x)]:
            for changed_y in [y, change_sign(y)]:
                for changed_z in [z, change_sign(z)]:
                    variants.append(build_by_cols(changed_x, changed_y, changed_z))   
    return variants


def read_scanner(locations):
    local_mat = DefaultDict(int)
    local_mat[(0,0,0)] = 'S'
    for loc in locations:
        local_mat[(loc[0], loc[1], loc[2])] = 'B'
    return local_mat

def find_overlapping(s1, s2):
    for x1,y1,z1 in s1.keys():
        for x2,y2,z2 in s2.keys():
            # We assume (x1,y1,z1) and (x2,y2,z2) are the same Beacon
            diff_x = x2 - x1
            diff_y = y2 - y1
            diff_z = z2 - z1
            s1_copy = s1.copy()
            for x2,y2,z2 in s2.keys():
                s1_copy[(x2 - diff_x, y2 - diff_y, z2-diff_z)] = s2[(x2,y2,z2)]
            # If we found the overlapping, we create the new matrix
            if len(s1) + len(s2) - MIN_BEACOUN_COUNT >= len(s1_copy):
                return s1_copy
    return False

def overlap(s1, locations2):
    for loc2 in get_variants(locations2):
        s2 = read_scanner(loc2)
        overlap = find_overlapping(s1,s2)
        if overlap: return overlap

def overlap_all_locations(locations):
    s1 = read_scanner(locations[0])
    added = set([0])
    
    while len(added) != len(locations):
        for idx in range(1,len(locations)):
            locations2 = locations[idx]
            local_s1 = overlap(s1, locations2)
            if local_s1:
                s1 = local_s1
                added.add(idx)
    return s1

def count_beacon(beacon_map):
    return len([v for v in beacon_map.values() if v == 'B'])

def get_manhattan(loc1, loc2):
    return abs(loc1[0] - loc2[0]) + abs(loc1[1] - loc2[1]) + abs(loc1[2] - loc2[2])

def find_largest_manhattan(beacon_map):
    sensors = [key for key,v in beacon_map.items() if v == 'S']
    largest_m = float("-inf")
    for s1 in sensors:
        for s2 in sensors:
            largest_m = max([largest_m, get_manhattan(s1,s2)])
    return largest_m

locations = []
loc = []
for idx, line in enumerate(fileinput.input()):
    line = line.strip()
    if "---" in line:
        if loc: locations.append(loc)
        loc = []
    elif line:
        values = [int(val) for val in line.split(',')]
        loc.append((values[0],values[1],values[2]))
locations.append(loc)

beacon_map = overlap_all_locations(locations)

# First Star

print(count_beacon(beacon_map))

# Second Star

print(find_largest_manhattan(beacon_map))