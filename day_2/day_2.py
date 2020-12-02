import fileinput

valid_passwords = 0

# First Star

for line in fileinput.input():
  line_elements = line.split(" ")
  policy_range = [int(x) for x in line_elements[0].split('-')]
  policy_letter = line_elements[1][0]
  password = line_elements[2]

  policy_letter_count = 0
  for char in password:
    if char == policy_letter:
      policy_letter_count += 1

  if (policy_letter_count >= policy_range[0] and policy_letter_count <= policy_range[1]):
    valid_passwords += 1

print(valid_passwords)

# Second Star

for line in fileinput.input():
  line_elements = line.split(" ")
  policy_positions = [int(x) for x in line_elements[0].split('-')]
  policy_letter = line_elements[1][0]
  password = line_elements[2]

  if (password[policy_positions[0] - 1] == policy_letter and password[policy_positions[1] - 1] != policy_letter):
    valid_passwords += 1
  
  if (password[policy_positions[0] - 1] != policy_letter and password[policy_positions[1] - 1] == policy_letter):
    valid_passwords += 1

print(valid_passwords)
  