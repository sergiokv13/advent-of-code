import fileinput

lanterfish = {}

def inc_dict(my_dict, el, val):
  if my_dict.get(el): my_dict[el] += val
  else: my_dict[el] = val

for line in fileinput.input():
  elems = [int(n) for n in line.split(',')]
for el in elems:
  inc_dict(lanterfish, el, 1)

def simulate(days, lanterfish):
  for _i in range(days):
    if (lanterfish.get(0)):
      inc_dict(lanterfish, 9, lanterfish.get(0))
      inc_dict(lanterfish, 7, lanterfish.get(0))
      lanterfish[0] = 0

    l_copy = lanterfish.copy()
    for day, count in l_copy.items():
      if (count and day):
        inc_dict(lanterfish, day, count * -1)
        inc_dict(lanterfish, day-1, count)
    
  return sum([c for _k,c in lanterfish.items()])

# # First Star
print(simulate(80, lanterfish))

# # Second Star

print(simulate(256, lanterfish))
