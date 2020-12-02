

import fileinput

##################
# Two values sum #
##################

values = {} 

for line in fileinput.input():
  val = int(line)
  needed = 2020 - val
  if needed in values:
    print(needed * val)
  else:
    values[val] = True

####################
# Three values sum #
####################

values = []
for line in fileinput.input():
  values.append(int(line))
 
values.sort()

for c in range(len(values) - 1):
  left_index = c + 1
  right_index = len(values) - 1

  while(left_index < right_index):
    local_sum = values[c] + values[left_index] + values[right_index]
    if (local_sum == 2020):
      print(values[c] * values[left_index] * values[right_index])
      break
    elif local_sum < 2020:
      left_index += 1
    else:
      right_index -= 1
