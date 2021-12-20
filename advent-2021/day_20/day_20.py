from collections import defaultdict
import fileinput
from typing import DefaultDict

image = dict() # We will save positions that store a # (y,x)
enhancement_algo = []
for idx, line in enumerate(fileinput.input()):
    if idx == 0:
        enhancement_algo = line.strip()
    if idx > 1:
        for x, el in enumerate(line.strip()):
            if (el == '#'): image[(idx-1,x)] = '1'
            else: image[(idx-1,x)] = '0'

# image section representation
#  123
#  456
#  789

def enhance_image(image):
    image_copy = image.copy()

    for y,x in image.copy().keys():
        # when pos is on 1
        new_val = enhancement_algo[int((image_copy[(y,x)] + image_copy[(y,x+1)] + image_copy[(y,x+2)] 
        + image_copy[(y+1,x)] + image_copy[(y+1,x+1)] + image_copy[(y+1,x+2)]
        + image_copy[(y+2,x)] + image_copy[(y+2,x+1)] + image_copy[(y+2,x+2)]), 2)]
        if (new_val == '#'): image[(y+1,x+1)] = '1'
        else: image[(y+1,x+1)] = '0'

        # when pos is on 2
        new_val = enhancement_algo[int((image_copy[(y,x-1)] + image_copy[(y,x)] + image_copy[(y,x+1)] 
        + image_copy[(y+1,x-1)] + image_copy[(y+1,x)] + image_copy[(y+1,x+1)]
        + image_copy[(y+2,x-1)] + image_copy[(y+2,x)] + image_copy[(y+2,x+1)]), 2)]
        if (new_val == '#'): image[(y+1,x)] = '1'
        else: image[(y+1,x)] = '0'

        # when pos is on 3
        new_val = enhancement_algo[int((image_copy[(y,x-2)] + image_copy[(y,x-1)] + image_copy[(y,x)] 
        + image_copy[(y+1,x-2)] + image_copy[(y+1,x-1)] + image_copy[(y+1,x)]
        + image_copy[(y+2,x-2)] + image_copy[(y+2,x-1)] + image_copy[(y+2,x)]), 2)]
        if (new_val == '#'): image[(y+1,x-1)] = '1'
        else: image[(y+1,x-1)] = '0'

       # when pos is on 4
        new_val = enhancement_algo[int((image_copy[(y-1,x)] + image_copy[(y-1,x+1)] + image_copy[(y-1,x+2)] 
        + image_copy[(y,x)] + image_copy[(y,x+1)] + image_copy[(y,x+2)]
        + image_copy[(y+1,x)] + image_copy[(y+1,x+1)] + image_copy[(y+1,x+2)]), 2)]
        if (new_val == '#'): image[(y,x+1)] = '1'
        else: image[(y,x+1)] = '0'

        # when pos is on 5
        new_val = enhancement_algo[int((image_copy[(y-1,x-1)] + image_copy[(y-1,x)] + image_copy[(y-1,x+1)] 
        + image_copy[(y,x-1)] + image_copy[(y,x)] + image_copy[(y,x+1)]
        + image_copy[(y+1,x-1)] + image_copy[(y+1,x)] + image_copy[(y+1,x+1)]), 2)]
        if (new_val == '#'): image[(y,x)] = '1'
        else: image[(y,x)] = '0'

        # when pos is on 6
        new_val = enhancement_algo[int((image_copy[(y-1,x-2)] + image_copy[(y-1,x-1)] + image_copy[(y-1,x)] 
        + image_copy[(y,x-2)] + image_copy[(y,x-1)] + image_copy[(y,x)]
        + image_copy[(y+1,x-2)] + image_copy[(y+1,x-1)] + image_copy[(y+1,x)]), 2)]
        if (new_val == '#'): image[(y,x-1)] = '1'
        else: image[(y,x-1)] = '0'

       # when pos is on 7
        new_val = enhancement_algo[int((image_copy[(y-2,x)] + image_copy[(y-2,x+1)] + image_copy[(y-2,x+2)] 
        + image_copy[(y-1,x)] + image_copy[(y-1,x+1)] + image_copy[(y-1,x+2)]
        + image_copy[(y,x)] + image_copy[(y,x+1)] + image_copy[(y,x+2)]), 2)]
        if (new_val == '#'): image[(y-1,x+1)] = '1'
        else: image[(y-1,x+1)] = '0'

        # when pos is on 8
        new_val = enhancement_algo[int((image_copy[(y-2,x-1)] + image_copy[(y-2,x)] + image_copy[(y-2,x+1)] 
        + image_copy[(y-1,x-1)] + image_copy[(y-1,x)] + image_copy[(y-1,x+1)]
        + image_copy[(y,x-1)] + image_copy[(y,x)] + image_copy[(y,x+1)]), 2)]
        if (new_val == '#'): image[(y-1,x)] = '1'
        else: image[(y-1,x)] = '0'

        # when pos is on 9
        new_val = enhancement_algo[int((image_copy[(y-2,x-2)] + image_copy[(y-2,x-1)] + image_copy[(y-2,x)] 
        + image_copy[(y-1,x-2)] + image_copy[(y-1,x-1)] + image_copy[(y-1,x)]
        + image_copy[(y,x-2)] + image_copy[(y,x-1)] + image_copy[(y,x)]), 2)]
        if (new_val == '#'): image[(y-1,x-1)] = '1'
        else: image[(y-1,x-1)] = '0'

    return image

def print_image(image):
    print(image)
    for y in range(-10,10):
        a = ''
        for x in range(-10,10):
            if image[(y,x)] == '1': a += '#'
            else: a += '.'
        print(a)
    print()

def enhance_n(n, image):
    for i in range(n):
        if (enhancement_algo[0] == '.'): default_val = '0'
        else: default_val = '0' if i%2 == 0 else '1'

        new_img = defaultdict(lambda:default_val)
        for pos,val in image.items():
            new_img[pos] = val
        image = enhance_image(new_img)
    
    return len([val for val in image.values() if val == '1'])

# First Star

print(enhance_n(2, image))

# Second Star

print(enhance_n(50, image))

