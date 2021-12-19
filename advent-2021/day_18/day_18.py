import fileinput
import re
import math
from collections import Counter


def get_level(list_str, init, end):
    list_str = re.sub("\d+", "", list_str[:init]).replace(',', '') + list_str[init:end] + re.sub("\d+", "", list_str[end:]).replace(',', '')
    while('[]' in list_str):
        list_str = list_str.replace('[]', '')
    return Counter(list_str)['['] - 1

def explode(list_str):
    prev_list_str = list_str[:]
    matches = re.finditer("\d+,\d+", list_str)
    for match in matches:
        init, end = match.span()        
        x, y = eval(f"({match.group()})")
        if get_level(list_str, init, end ) >= 4:
            zero_idx = init-1
            list_str = list_str[:init-1] + '0' + list_str[end+1:]
            # find last number at left
            left_matches = list(re.finditer("\d+", list_str[0:zero_idx]))
            left_init, left_end = None, None
            right_init, right_end = None, None
            if left_matches:
                left_match = left_matches[-1]
                left_init, left_end = left_match.span()
                left_value = int(left_match.group())
                left_value += x
            
            # find last number at right
            right_matches = list(re.finditer("\d+", list_str[zero_idx+1:len(list_str)]))
            if right_matches:
                right_match = right_matches[0]
                right_init, right_end = right_match.span()
                right_init += zero_idx+1 ; right_end += zero_idx+1
                right_value = int(right_match.group())
                right_value += y

            if (left_init is not None and right_init is not None):
                list_str = list_str[:left_init] + str(left_value) + list_str[left_end:right_init] + str(right_value) + list_str[right_end:]
            elif left_init is not None:
                list_str = list_str[:left_init] + str(left_value) + list_str[left_end:]
            else:
                list_str = list_str[:right_init] + str(right_value) + list_str[right_end:]

            
            break # Only first par will be exploded
        
    return list_str, list_str != prev_list_str

        
def split(list_str):
    prev_list_str = list_str[:]
    matches = re.finditer("\d+", list_str)
    for match in matches:
        regular_number = int(match.group())
        init, end = match.span()
        if regular_number >= 10:
            x = regular_number // 2
            y = math.ceil(regular_number / 2)
            xy_str = f"[{x},{y}]"
            list_str = list_str[:init] + str(xy_str) + list_str[end:]
            break # Only first number will be splitted
    return list_str, list_str != prev_list_str

def add(list_str1, list_str2):
    list_str = f"[{list_str1},{list_str2}]"
    exploded = True
    splitted = True
    while(exploded or splitted):
            list_str, exploded = explode(list_str)
            if exploded: continue
            list_str, splitted = split(list_str)
        
    return list_str

def get_magnitude(list_str):
    matches = list(re.finditer("\d+,\d+", list_str))
    
    if not matches:
        return int(list_str)

    for match in matches:
        init, end = match.span()        
        x, y = eval(f"({match.group()})")
        value = x*3 + y*2
        list_str = list_str[:init-1] + str(value) + list_str[end+1:]
        break # Indexes will change, so let's go one by one

    return get_magnitude(list_str)

list_str = None
sm_numbers = []
for idx, line in enumerate(fileinput.input()):
    if idx == 0:
        sm_numbers.append(line.strip())
        list_str = line.strip()
    if idx > 0:
        sm_numbers.append(line.strip())
        list_str = add(list_str, line.strip())


max_sum = float("-inf")
for el in sm_numbers:
    for el2 in sm_numbers:
        max_sum = max([max_sum, get_magnitude(add(el, el2))])
        max_sum = max([max_sum, get_magnitude(add(el2, el))])

# First Star

print(get_magnitude(list_str))

# Second Star

print(max_sum)
