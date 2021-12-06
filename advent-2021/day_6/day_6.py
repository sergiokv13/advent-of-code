import fileinput

lanterfish = [0] * 10

for line in fileinput.input():
  elems = [int(n) for n in line.split(',')]
for el in elems:
 lanterfish[el] += 1

def simulate(days, lanterfish):
  for _i in range(days):
    if (lanterfish[0]):
      lanterfish[9] += lanterfish[0]
      lanterfish[7] += lanterfish[0]
      lanterfish[0] = 0

    l_copy = lanterfish.copy()
    for day, count in enumerate(l_copy):
      if (count and day):
        lanterfish[day] -= count 
        lanterfish[day-1] += count
    
  return sum(lanterfish)

# # First Star
print(simulate(80, lanterfish))

# # # Second Star

print(simulate(256, lanterfish))
