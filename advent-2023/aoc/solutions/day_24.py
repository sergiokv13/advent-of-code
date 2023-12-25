from collections import defaultdict
from itertools import combinations
from scipy.optimize import fsolve
import numpy as np

def read_input():
  return [
    [
      [int(n) for n in el.split(", ")]
      for el in row.split(" @ ")
    ]
    for row in open("../inputs/day_24.input", "r").read().split("\n")
  ]


def check_intersection(hailstone1, hailstone2, area):
  [min_a, max_a] = area

  [[x,y,_],[vx,vy,_]] = hailstone1
  [[x2,y2,_],[vx2,vy2,_]] = hailstone2
  
  a = np.array([[vx, -1 * vx2], [vy, -1 * vy2]])
  b = np.array([x2  - x, y2 - y])

  try:
    [t1, t2] = np.linalg.solve(a, b)
    
    int_x = x + vx * t1
    int_y = y + vy * t1

    return (
      t1 >= 0 and t2 >= 0 and 
      (min_a <= int_x <= max_a) and 
      (min_a <= int_y <= max_a)
    )
  except:
    return False

def first_star():
  area = [200000000000000, 400000000000000]
  hailstones = read_input()

  intersections = [
    check_intersection(h[0], h[1], area) for h in
    combinations(hailstones, 2)
  ]
  print(sum(intersections))


def second_star():
  hailstones = read_input()

  def get_equations(vars, hailstones):
    [[hs1x,hs1y,hs1z], [hs1vx,hs1vy,hs1vz]] = hailstones[0]
    [[hs2x,hs2y,hs2z], [hs2vx,hs2vy,hs2vz]] = hailstones[1]
    [[hs3x,hs3y,hs3z], [hs3vx,hs3vy,hs3vz]] = hailstones[2]

    x, y, z, vx, vy, vz, t, t2, t3 = vars
    
    # for hs1
    eq1 = x + vx * t - (hs1x + hs1vx * t)
    eq2 = y + vy * t - (hs1y + hs1vy * t)
    eq3 = z + vz * t - (hs1z + hs1vz * t)

    # for hs2
    eq4 = x + vx * t2 - (hs2x + hs2vx * t2)
    eq5 = y + vy * t2 - (hs2y + hs2vy * t2)
    eq6 = z + vz * t2 - (hs2z + hs2vz * t2)

    # for hs3
    eq7 = x + vx * t3 - (hs3x + hs3vx * t3)
    eq8 = y + vy * t3 - (hs3y + hs3vy * t3)
    eq9 = z + vz * t3 - (hs3z + hs3vz * t3)
  
    return [eq1, eq2, eq3, eq4, eq5, eq6, eq7, eq8, eq9]
  
  # iterating over all the hailstones 100 times to approximate the value
  solution = [0] * 9
  
  solution_count = defaultdict(int)
  for _ in range(0,100):
    for idx in range(0, len(hailstones)-2):
      solution = fsolve(
        lambda vars:get_equations(vars, hailstones[idx:idx+3]), 
        solution,
        maxfev=1_000_000,
        factor=1,
      )
    [x, y, z, vx, vy, vz, t, t2, t3] = solution
    solution = [round(x), round(y), round(z), round(vx), round(vy), round(vz), round(t), round(t2), round(t3)]
    solution_count[round(x) + round(y) + round(z)] += 1

  solution_count.items()
  # get most frequent solution
  print(max(solution_count.items(), key=lambda c: c[1])[0])

first_star()
second_star()