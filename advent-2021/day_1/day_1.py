import fileinput

# First Star

prev_val = None
increased = 0
for line in fileinput.input():
  val = int(line)
  if (prev_val and prev_val < val): increased += 1
  prev_val = val

print(increased)


# Second Star

values = []
increased = 0
for line in fileinput.input():
  val = int(line)
  values.append(val)

prev_sum = 0
current_sum = 0
for i in range(len(values) - 2):
  current_sum = sum(values[i:i+3])
  if (prev_sum and prev_sum < current_sum): increased += 1
  prev_sum = current_sum

print(increased)