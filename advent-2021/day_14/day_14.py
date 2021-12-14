import fileinput
from typing import DefaultDict
import math

def get_letter_count(steps, template, rules):
    pairs_count = DefaultDict(int)
    for i in range(len(template) - 1):
        pairs_count[template[i] + template[i+1]] += 1
    
    for i in range(steps):
        pairs_count_copy = pairs_count.copy()
        for pair in pairs_count_copy.keys():
            # original pair will be removed
            pairs_count[pair] -= pairs_count_copy[pair]
            for new_pair in rules[pair]:
                # add as many new pairs as the base one
                pairs_count[new_pair] += pairs_count_copy[pair]
                if (pairs_count[pair] == 0): del pairs_count[pair]
        
    # count occurences
    occ_count = DefaultDict(int)
    for pair in pairs_count.keys():
        occ_count[pair[0]] += pairs_count[pair]
        occ_count[pair[1]] += pairs_count[pair]
    
    max_val = float("-inf")
    min_val = float("inf")
    for letter in occ_count.keys():
        val = math.ceil(occ_count[letter] / 2)
        if val > max_val: max_val = val
        if val < min_val: min_val = val

    return max_val, min_val


rules = DefaultDict(list) # Pair -> [Resulting new pairs] 
template = None

for idx, line in enumerate(fileinput.input()):
    if idx == 0:
        template = line.strip()
    if idx > 1:
       init_pair, to_add = line.strip().split(" -> ")
       rules[init_pair].append(init_pair[0] + to_add)
       rules[init_pair].append(to_add + init_pair[1])

# First Star

max_val, min_val = get_letter_count(10, template, rules)
print(max_val - min_val)

# Second Star

max_val, min_val = get_letter_count(40, template, rules)
print(max_val - min_val)