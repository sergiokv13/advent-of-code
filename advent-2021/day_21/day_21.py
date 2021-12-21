import fileinput
import itertools

def throw_det_dice_func():
    turns = 0
    while True:
        for i in range(1,101):
            turns += 1
            yield i, turns

throw_det_dice = throw_det_dice_func()

def get_next_move():
    throws = list(itertools.islice(throw_det_dice, 3))
    return sum([t[0] for t in throws]), throws[-1][1]

def simulate_game(p1pos, p2pos):
    p1score = 0
    p2score = 0

    while(True):
        # Player one turn
        move, throws = get_next_move()
        p1pos = (move + p1pos) % 10 or 10
        p1score += p1pos
        if (p1score >= 1000): return throws * p2score

        # Player two turn
        move, throws = get_next_move()
        p2pos = (move + p2pos) % 10 or 10
        p2score += p2pos
        if (p2score >= 1000): return throws * p1score

memo = {}
dice_rolls = [sum(p) for p in itertools.product([1,2,3], repeat=3)]
def simulate_quantum_game(p1pos, p1score, p2pos, p2score):
    if (p1pos, p1score, p2pos, p2score) in memo:
        return memo[(p1pos, p1score, p2pos, p2score)]

    p1_universes = 0
    p2_universes = 0
    for move1 in dice_rolls:
        localp1pos = p1pos
        localp1score = p1score
        
        localp1pos = (move1 + localp1pos) % 10 or 10
        localp1score += localp1pos
        if (localp1score >= 21): 
            p1_universes += 1
            continue

        for move2 in dice_rolls:
            localp2pos = p2pos
            localp2score = p2score

            localp2pos = (move2 + localp2pos) % 10 or 10
            localp2score += localp2pos
            if (localp2score >= 21): 
                p2_universes += 1
                continue

            childp1_universes, childp2_universes = simulate_quantum_game(localp1pos, localp1score, localp2pos, localp2score)
            p1_universes += childp1_universes
            p2_universes += childp2_universes

    memo[(p1pos, p1score, p2pos, p2score)] = (p1_universes, p2_universes)
    return memo[(p1pos, p1score, p2pos, p2score)]

pos1 = None
pos2 = None
for idx, line in enumerate(fileinput.input()):
    if idx == 0:
        pos1 = int(line.strip().replace("Player 1 starting position: ", ""))
    if idx == 1:
        pos2 = int(line.strip().replace("Player 2 starting position: ", ""))

# First Star

print(simulate_game(pos1, pos2))

# Second Star

print(max(list(simulate_quantum_game(pos1, 0, pos2, 0))))

