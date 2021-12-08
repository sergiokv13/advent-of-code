import fileinput
from collections import defaultdict

# patterns: number
needed_wires = { 2: [1], 3: [7], 4: [4], 5: [2, 3, 5], 6: [0, 6 , 9], 7: [8] }

# counter for numbers
count_easy = defaultdict(int)

four_digit_sum = 0

def count_easy_digits(coded_number):
  for coded_digit in coded_number:
    if len(needed_wires[len(coded_digit)]) == 1:
      count_easy[needed_wires[len(coded_digit)][0]] += 1


def decode_number(coded_number, connections):
  number_1 = set([x for x in connections if len(x) == 2][0])
  number_4 = set([x for x in connections if len(x) == 4][0])
  number_7 = set([x for x in connections if len(x) == 3][0])
  number_8 = set([x for x in connections if len(x) == 7][0])
  number_3 = set([x for x in connections if len(x) == 5 and len(set(x) - number_1) == 3][0])
  number_5 = set([x for x in connections if len(x) == 5 and len(set(x) - number_3 - number_4) == 1 and set(x) != number_3][0])
  number_2 = set([x for x in connections if len(x) == 5 and set(x) != number_5 and set(x) != number_3][0])
  number_9 = set([x for x in connections if len(x) == 6 and len(set(x) - number_3) == 1][0])
  number_0 = set([x for x in connections if len(x) == 6 and set(x) != number_9 and len(set(x) - number_1) == 4][0])
  number_6 = set([x for x in connections if len(x) == 6 and set(x) != number_0 and set(x) != number_9][0])
  
  # get coded_number
  number_list = []
  for coded_digit in coded_number:
    if (set(coded_digit) == number_1): number_list.append('1')
    if (set(coded_digit) == number_2): number_list.append('2')
    if (set(coded_digit) == number_3): number_list.append('3')
    if (set(coded_digit) == number_4): number_list.append('4')
    if (set(coded_digit) == number_5): number_list.append('5')
    if (set(coded_digit) == number_6): number_list.append('6')
    if (set(coded_digit) == number_7): number_list.append('7')
    if (set(coded_digit) == number_8): number_list.append('8')
    if (set(coded_digit) == number_9): number_list.append('9')
    if (set(coded_digit) == number_0): number_list.append('0')

  return int(('').join(number_list))
  

for line in fileinput.input():
  connections = line.split('|')[0].strip().split(" ")
  coded_number = line.split('|')[1].strip().split(" ")
  count_easy_digits(coded_number)

  four_digit_sum += decode_number(coded_number, connections)

# # First Star

print(sum(count_easy.values()))

# Second Star

print(four_digit_sum)

