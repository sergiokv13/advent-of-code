import fileinput
from statistics import mode
from collections import Counter

def get_power_consumption(bit_mat):
  gamma = []
  epsilon = []
  for i in range(len(bit_mat[0])):
    common_bit = mode([row[i] for row in bit_mat])
    uncommon_bit = '0' if common_bit == '1' else '1'

    gamma.append(common_bit)
    epsilon.append(uncommon_bit)
  

  return int(''.join(gamma),2) * int(''.join(epsilon),2)
    
def get_oxygen_gen(bit_mat):
  local_mat = bit_mat.copy()
  bit_idx = 0

  while(len(local_mat) > 1):
    occ_count = Counter([row[bit_idx] for row in local_mat])
    if (occ_count['1'] == occ_count['0']): bit_to_filter = '1'
    else: bit_to_filter = occ_count.most_common(1)[0][0]
    local_mat = [row for row in local_mat if row[bit_idx] == bit_to_filter]
    bit_idx += 1
  return int(''.join(local_mat[0]),2)

def get_co2(bit_mat):
  local_mat = bit_mat.copy()
  bit_idx = 0

  while(len(local_mat) > 1):
    occ_count = Counter([row[bit_idx] for row in local_mat])
    if (occ_count['1'] == occ_count['0']): bit_to_filter = '0'
    else: bit_to_filter = occ_count.most_common()[-1][0]
    local_mat = [row for row in local_mat if row[bit_idx] == bit_to_filter]
    bit_idx += 1
  return int(''.join(local_mat[0]),2)
  
bit_mat = []
for line in fileinput.input():
  bit_mat.append(line.strip())

# First Star

print(get_power_consumption(bit_mat))

# Second Star

print(get_co2(bit_mat) * get_oxygen_gen(bit_mat))