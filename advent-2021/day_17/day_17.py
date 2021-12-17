import fileinput
from typing import DefaultDict
import math 

def still_valid(posxy, x_range, y_range):
    x,y = posxy
    if (x >= x_range[0] and x <= x_range[1]) and (y >= y_range[0] and y <= y_range[1]): return "success"
    if x > max(x_range): return "fail"
    if y < min(y_range): return "fail"
    return "flying"

def simulate(throw, x_range, y_range):
    current_position = [0,0]
    current_throw = throw
    max_y = float("-inf")
    while still_valid(current_position, x_range, y_range) == "flying":
        current_x, current_y = current_position
        current_x_throw, current_y_throw = current_throw
        # Move
        current_x += current_x_throw
        current_y += current_y_throw
        # Store max height
        if current_y > max_y : max_y = current_y
        # Recalculate throw
        if current_x_throw > 0: current_x_throw -= 1
        if current_x_throw < 0: current_x_throw += 1
        current_y_throw -= 1

        current_position = [current_x, current_y]
        current_throw = [current_x_throw, current_y_throw]

    success = still_valid(current_position, x_range, y_range) == "success"
    return success, max_y

def get_star_values(x_range, y_range):
    max_height = float("-inf")
    valid_initial = []
    for x in range(max(x_range)*2):
        for y in range(-100,100):
            success, height = simulate([x,y], x_range, y_range)
            if (success): 
                max_height = max([max_height, height])
                valid_initial.append((x,y))
    return max_height, len(valid_initial)

x_range = None
y_range = None
for line in fileinput.input():
    line = line.strip().replace("target area: ", "")
    x_raw, y_raw = line.split(", ")
    x_range = [int(el) for el in x_raw.replace("x=","").split('..')]
    y_range = [int(el) for el in y_raw.replace("y=","").split('..')]

star1, star2 = get_star_values(x_range, y_range)

# First Star

print(star1)

# Second Star

print(star2)
