import fileinput

def process_instruction(instruction, w, x, y, z):
    op, v1, v2 = instruction.split()
    _locals = locals()
    if op == "add":
        exec(f"{v1} = {v1} + {v2}", globals(), _locals)
    if op == "mul":
        exec(f"{v1} = {v1} * {v2}", globals(), _locals)
    if op == "div":
        exec(f"{v1} = {v1} // {v2}", globals(), _locals)
    if op == "mod":
        exec(f"{v1} = {v1} % {v2}", globals(), _locals)
    if op == "eql":
        exec(f"{v1} = int({v1} == {v2})", globals(), _locals)
    
    return _locals['w'], _locals['x'], _locals['y'], _locals['z']

# all inputs are stored on w registry
def process_alu_instructions(instructions, w, x = 0, y = 0, z = 0):
    for instruction in instructions:
        w, x, y, z = process_instruction(instruction, w, x, y, z)
    return w, x, y, z


z_memo = {} # only z change between steps
def find_monad(instructions_step, biggest = True, current = "", registries = {'w': 0, 'x': 0, 'y': 0, 'z': 0}):
    w, x, y, z = registries['w'], registries['x'], registries['y'], registries['z']

    # If we have 14 numbers, we finished searching
    if len(current) == 14:
        print(current)
        return int(current) if z == 0 else "invalid"

    # add new number
    finished_solution = "invalid"
    # range will start on 9 if biggest, on 1 if not
    search_range = range(9,0,-1) if biggest else range(1,10)
    for number in search_range:
        w = number # w resets everytime we read a new number
        wc, xc, yc, zc = process_alu_instructions(instructions_step[len(current)], w, x, y, z)

        new_current = current + str(number)

        # check if we already visited the z state
        if (zc, len(new_current)) not in z_memo:
            sol = find_monad(instructions_step, biggest, new_current, { 'w': wc, 'x': xc, 'y': yc, 'z': zc })

            if sol != "invalid":
                finished_solution = sol
                break

    z_memo[(registries['z'], len(current))] = finished_solution

    return finished_solution

instructions_step = []
instruction_local = []
for idx, line in enumerate(fileinput.input()):
    if "inp" in line:
        if instruction_local:
            instructions_step.append(instruction_local)
            instruction_local = []
    else:
        instruction_local.append(line.strip())
instructions_step.append(instruction_local)


# First Star

print(find_monad(instructions_step, biggest = True))

# Second Star

print(find_monad(instructions_step, biggest = False))




