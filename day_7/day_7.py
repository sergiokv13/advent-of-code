import fileinput
import functools
import re
import math

has_shiny_dp = {}
def has_shiny(bag, bags_graph):
  if bag in has_shiny_dp:
    return has_shiny_dp[bag]

  res = False
  if (bag == 'shiny gold'):
    res = True
  else:
    for bag_child in bags_graph[bag]:
      if has_shiny(bag_child[0], bags_graph):
        res = True

  has_shiny_dp[bag] = res
  return has_shiny_dp[bag]

count_bags_dp = {}
def count_bags(bag, bags_graph):
  if bag in count_bags_dp:
    return count_bags_dp[bag]

  res = 1
  for bag_child in bags_graph[bag]:
    res += int(bag_child[1]) * count_bags(bag_child[0], bags_graph)

  count_bags_dp[bag] = res
  return count_bags_dp[bag]

# Build Graph
bags_graph = {}
bags = set()
for line in fileinput.input():
  clean_line = line.replace('bags', '').replace('bag', '').replace(',', '').replace('.', '').replace('contain', '')
  clean_line = re.sub(' +', ' ', clean_line).strip().split(' ')
  main_bag = ' '.join(clean_line[:2])
  bags_graph[main_bag] = []

  bags.add(main_bag)

  idx = 2
  while idx < len(clean_line[2:]):
    if re.match('^[0-9]+$', clean_line[idx]):
      child_bag = ' '.join(clean_line[idx + 1:idx + 3])
      bags_graph[main_bag].append((child_bag, clean_line[idx]))
      bags.add(child_bag)
      idx = idx + 3

# First Star
shiny_count = 0

# removing shiny gold because we need the ones that contains it
for bag in bags - set(['shiny gold']):
  shiny_count += has_shiny(bag, bags_graph)

print(shiny_count)

# Second Start
# removing 1 to not count the initial bag
print(count_bags('shiny gold', bags_graph) - 1)
print()