import fileinput
import functools
import re
import math

# First Star

group_ans = set()
yes_sum = 0
for line in fileinput.input():
  if (line == '\n'):
    yes_sum += len(group_ans)
    group_ans = set()
  else:
    for char in line.strip():
      group_ans.add(char)

if group_ans:
  yes_sum += len(group_ans)

print(yes_sum)

# Second Star

def check_group_questions(question_sum, number_of_people):
  yes_sum = 0
  for question in question_sum.values():
        if question == number_of_people:
          yes_sum += 1
  return yes_sum

yes_sum = 0
question_sum = {}
number_of_people = 0
for line in fileinput.input():
  if (line == '\n'):
    yes_sum += check_group_questions(question_sum, number_of_people)
    number_of_people = 0
    question_sum = {}
  else:
    number_of_people += 1
    for char in line.strip():
      if char in question_sum:
        question_sum[char] += 1
      else:
        question_sum[char] = 1

if question_sum:
  yes_sum += check_group_questions(question_sum, number_of_people)

print(yes_sum)
