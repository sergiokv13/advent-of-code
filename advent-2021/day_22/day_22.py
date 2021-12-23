import fileinput

def valid_range(xr, yr, zr):
    if xr[0] > 51 or xr[0] < -50 or xr[1] > 51 or xr[1] < -50: return False
    if yr[0] > 51 or yr[0] < -50 or yr[1] > 51 or yr[1] < -50: return False
    if zr[0] > 51 or zr[0] < -50 or zr[1] > 51 or zr[1] < -50: return False

    return True

def overlap_axis(x1, x2):
    x1min, x1max = x1
    x2min, x2max = x2
    
    # I'm pretty sure there is a better way to do this, but I'm too tired
    if (
        (x1min > x2min and x1min < x2max) 
        or (x1max < x2max and x1max > x2min) 
        or ((x1min >= x2min and x1min <= x2max) and (x1max <= x2max and x1max >= x2min))
    ):
        max_min = x1min if x1min > x2min else x2min
        min_max = x1max if x1max < x2max else x2max
        return [max_min, min_max]
    return False

def get_axis_segments(x1, ox):
    xarr = []; 
    xmin, xmax = x1; 
    oxmin, oxmax = ox
    if (oxmin >= xmin):
        xarr.append([xmin, oxmin])
        if (oxmax >= xmax): xarr.append([oxmin, xmax])        
        else: xarr.append([oxmin,oxmax]); xarr.append([oxmax, xmax])
    elif (oxmax <= xmax):
        xarr.append([xmin, oxmax])
        xarr.append([oxmax, xmax])

    return [el for el in xarr if el[0] != el[1]]

def check_overlap(b1, b2):
    ox = overlap_axis(b1[0], b2[0]) or overlap_axis(b2[0], b1[0])
    oy = overlap_axis(b1[1], b2[1]) or overlap_axis(b2[1], b1[1])
    oz = overlap_axis(b1[2], b2[2]) or overlap_axis(b2[2], b1[2])

    if ox and oy and oz:
        new_cubes = []
        x_segments = get_axis_segments(b1[0], ox)
        y_segments = get_axis_segments(b1[1], oy)
        z_segments = get_axis_segments(b1[2], oz)

        # craete all the subcubes after making cuts on the x,y,z
        for xs in x_segments:
            for ys in y_segments:
                for zs in z_segments:
                    # check if this is part of the cube we will add later (or end up removing)
                    if (not (
                        overlap_axis(xs, b2[0]) and overlap_axis(ys, b2[1]) and overlap_axis(zs, b2[2])
                    )):
                        if (b2 != [xs, ys, zs]) and [xs, ys, zs] not in new_cubes:
                            new_cubes.append([xs, ys, zs])

        return new_cubes

    return False


def set_state(cubes, lrange, state):
    new_cubes = []
    for cube in cubes:
        overlapr = check_overlap(cube, lrange)

        # we remove the overlap and get new cubes
        if overlapr: new_cubes = new_cubes + overlapr
        elif overlapr is False: new_cubes.append(cube) # cube is already considered
    
    # if add, we always add the cube
    # if we are setting as off, we already removed lrange
    if state: 
        new_cubes.append(lrange)

    return new_cubes

def count_cubes(cubes):
    local_sum = 0
    visited = set()
    for cube in cubes:
        xr, yr, zr = cube
        local_sum += abs(xr[0]-xr[1]) * abs(yr[0]-yr[1]) * abs(zr[0]-zr[1])
    return local_sum

cubes = [] # [((x1,x2), (y1,y2), (z1,z2))]
cubes_first = []
for idx, line in enumerate(fileinput.input()):
    splitted = line.strip().split(',')
    xr = [int(x) for x in splitted[0].replace("on x=", "").replace("off x=", "").split("..")]
    yr = [int(y) for y in splitted[1].replace("y=", "").split("..")]
    zr = [int(z) for z in splitted[2].replace("z=", "").split("..")]

    # adding one, include first, exclude last
    xr[1] += 1
    yr[1] += 1
    zr[1] += 1

    state = 1 if "on" in line else 0
    # print(valid_range(xr, yr, zr))
    if valid_range(xr, yr, zr):
        cubes_first = set_state(cubes_first, [xr, yr, zr], state)

    
    cubes = set_state(cubes, [xr, yr, zr], state)


# First Star

print(count_cubes(cubes_first))

# # Second Star

print(count_cubes(cubes))