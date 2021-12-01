import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy

def get_possible_by_allergen(items):
  possible_allergens = {}
  ingredients_set = set()

  for item in items:
    splitted_item = item.replace('(','').replace(')','').split('contains')
    ingredients = [el.strip() for el in splitted_item[0].strip().split(' ')]
    ingredients_set = ingredients_set | set(ingredients)

    allergens = [el.strip() for el in splitted_item[1].strip().split(',')]
    for allergen in allergens:
      if allergen in possible_allergens:
        # get the intersection of ingredients that are for that allergen
        possible_allergens[allergen] = possible_allergens[allergen] & set(ingredients)
      else:
        possible_allergens[allergen] = set(ingredients)

  return possible_allergens, ingredients_set

# Returns tuple with ingredient,allergen_component
def get_allergens(possible_allergen):
  allergens = []
  while possible_allergen:
    # we can assume we will only have one possible_allergen of size 1 to get a solution
    for possible,ingredients in possible_allergen.items():
      if len(ingredients) == 1: 
        allergens.append((list(ingredients)[0], possible))
        del possible_allergen[possible]
        # Remove ingredient from all other
        for other_possible in possible_allergen:
          possible_allergen[other_possible] = possible_allergen[other_possible] - ingredients

        break

  return allergens

def count_non_allergen_appears(items, non_allergen):
  count = 0
  for item in items:
    splitted_item = item.replace('(','').replace(')','').split('contains')
    ingredients = [el.strip() for el in splitted_item[0].strip().split(' ')]
    count += len(set(ingredients) & non_allergen)

  return count


items = []
for line in fileinput.input():
  clean_line = line.strip()
  items.append(clean_line)

possible_allergen, ingredients = get_possible_by_allergen(items)
allergens = get_allergens(possible_allergen)

# First Star
non_allergens_ingredients = ingredients - set(ing for ing,al in allergens)
print(count_non_allergen_appears(items, non_allergens_ingredients))

# Second Star
sorted_allergens = sorted(allergens, key=lambda al: al[1])
print((',').join([ing for ing,al in sorted_allergens]))