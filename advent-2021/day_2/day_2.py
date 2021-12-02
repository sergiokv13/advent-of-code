import fileinput

class Submarine_star_1():
  def __init__(self):
    self.depth = 0
    self.horizontal_pos = 0

  def forward(self, val):
    self.horizontal_pos += val
  
  def down(self, val):
    self.depth += val

  def up(self, val):
    self.depth -= val

  def mult(self):
    return self.depth * self.horizontal_pos

  def call(self, method_name, params = None):
    func = getattr(self, method_name)
    func(params)
    
class Submarine_star_2():
  def __init__(self):
    self.depth = 0
    self.horizontal_pos = 0
    self.aim = 0

  def forward(self, val):
    self.horizontal_pos += val
    self.depth += val * self.aim
  
  def down(self, val):
    self.aim += val

  def up(self, val):
    self.aim -= val

  def mult(self):
    return self.depth * self.horizontal_pos

  def call(self, method_name, params = None):
    func = getattr(self, method_name)
    func(params)


submarine1 = Submarine_star_1()
submarine2 = Submarine_star_2()
for line in fileinput.input():
  line_elements = line.split(" ")
  instruction = line_elements[0]
  value = int(line_elements[1])
  submarine1.call(instruction, value)
  submarine2.call(instruction, value)

# # First Star
# print(submarine1.mult())

# Second Star
print(submarine2.mult())