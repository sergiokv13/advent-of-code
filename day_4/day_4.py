import fileinput
import functools
import re

# First Start
def valid_passport_first_star(passport_attributes):
  for attr in passport_attributes:
      key, val = attr.split(':')
      pkeys[key] = val

  required = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
  return all(k in pkeys for k in required)

# Second Star
def valid_number(pkeys, key, min_v, max_v):
  return (key in pkeys) and int(pkeys[key]) >= min_v and int(pkeys[key]) <= max_v

def valid_size(pkeys, key):
  if (key in pkeys):
    if pkeys[key][-2:] == 'cm' and int(pkeys[key][:-2]) >= 150 and int(pkeys[key][:-2]) <= 193:
      return True
    
    if pkeys[key][-2:] == 'in' and int(pkeys[key][:-2]) >= 59 and int(pkeys[key][:-2]) <= 76:
      return True

  return False

def valid_hex_color(pkeys, key):
  return (key in pkeys) and re.match("^#([a-f0-9]{6})$", pkeys[key])

def valid_fix_color(pkeys, key):
  return (key in pkeys) and pkeys[key] in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']

def valid_passport_number(pkeys, key):
  return (key in pkeys) and re.match("^([0-9]{9})$", pkeys[key])


# Second Star
def valid_passport_second_star(passport_attributes):
  for attr in passport_attributes:
      key, val = attr.split(':')
      pkeys[key] = val

  if not valid_number(pkeys, 'byr', 1920, 2002):
    return False

  if not valid_number(pkeys, 'iyr', 2010, 2020):
    return False
  
  if not valid_number(pkeys, 'eyr', 2020, 2030):
    return False
  
  if not valid_size(pkeys, 'hgt'):
    return False
  
  if not valid_hex_color(pkeys, 'hcl'):
    return False
  
  if not valid_fix_color(pkeys, 'ecl'):
    return False
  
  if not valid_passport_number(pkeys, 'pid'):
    return False

  return True

pkeys = {}
valid_passports = 0
passport_attrs = []
for line in fileinput.input():
  if (line != "\n"):
    passport_attrs = passport_attrs + line.strip().split(' ')
  else:    
    valid_passports += valid_passport_second_star(passport_attrs)
    pkeys = {}
    passport_attrs = []

# last line data
if (passport_attrs):
  valid_passports += valid_passport_second_star(passport_attrs)


print(valid_passports)

