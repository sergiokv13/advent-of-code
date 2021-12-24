import fileinput
import sys
sys.setrecursionlimit(100000)

hall = {} #{pos: 'A'}
rooms = [] # [[A,B], [B,C], ...]
hall_positions = [0,1,3,5,7,9,10]
room_positions = [2,4,6,8]
weights = {'A': 1, 'B': 10, 'C': 100, 'D': 1000}

def check_room(room, expected):
    for el in room:
        if el != expected: return False
    return True

def valid_room_to_hall_move(hall, rooms, room_id, el_id, hall_dest):
    room = rooms[room_id]

    # check if we have other elements in front
    if el_id != 0:
        for i in range(0,el_id):
            if room[i]: return False

    if room_id == 0 and check_room([el for el in rooms[0] if el is not None], 'A'): return False
    if room_id == 1 and check_room([el for el in rooms[1] if el is not None], 'B'): return False
    if room_id == 2 and check_room([el for el in rooms[2] if el is not None], 'C'): return False
    if room_id == 3 and check_room([el for el in rooms[3] if el is not None], 'D'): return False

    # check if we can move to destiny on the hall
    first_hall_pos = room_positions[room_id]
    for i in range(min([first_hall_pos, hall_dest]), max([first_hall_pos, hall_dest])+1):
        if i in hall: return False
    
    return True

def valid_hall_to_room_move(hall, rooms, hall_pos, room_id, el):
    room = rooms[room_id]
    # if room is full
    if None not in room: return False
    # if we have other invalid element in room
    for rel in room:
        if (rel and rel != el): return False

    if el == 'A' and room_id != 0: return False
    if el == 'B' and room_id != 1: return False
    if el == 'C' and room_id != 2: return False
    if el == 'D' and room_id != 3: return False

    destiniy_hall_pos = room_positions[room_id]
    for i in range(min([destiniy_hall_pos, hall_pos]), max([destiniy_hall_pos, hall_pos])+1):
        if i in hall and i is not hall_pos: return False

    return True

def insert_to_room(room, el):
    room = room.copy()
    for i in range(len(room)-1, -1, -1):
        if room[i] == None:
            room[i] = el
            return room, i

def check_complete(rooms):
    if (
        check_room(rooms[0], 'A')
        and check_room(rooms[1], 'B')
        and check_room(rooms[2], 'C')
        and check_room(rooms[3], 'D')
    ): return True
    return False
    

visited_memo={}
def get_min(hall, rooms):
    memo_key = (str(hall), str(rooms))

    if memo_key in visited_memo:
        return visited_memo[memo_key]

    if check_complete(rooms): 
        return 0

    best = float("inf")

    # from hallway to room
    for hall_pos, el in hall.items():
        for room_id, room in enumerate(rooms):
            # If we have a space in the room
            if valid_hall_to_room_move(hall, rooms, hall_pos, room_id, el):
                new_rooms = [room.copy() for room in rooms]
                new_hall = hall.copy()

                new_rooms[room_id], el_id = insert_to_room(room, el)
                del new_hall[hall_pos]

                steps = abs(room_positions[room_id] - hall_pos) + el_id + 1
                move_weight = weights[el] * steps

                best = min([move_weight + get_min(new_hall, new_rooms), best])

    # from room to hallway
    for room_id, room in enumerate(rooms):
        for el_id, el in enumerate(room):
            if el:
                for hall_pos in hall_positions:
                    if valid_room_to_hall_move(hall, rooms, room_id, el_id, hall_pos):
                        new_rooms = [room.copy() for room in rooms]
                        new_hall = hall.copy()

                        new_rooms[room_id][el_id] = None
                        new_hall[hall_pos] = el

                        steps = abs(room_positions[room_id] - hall_pos) + el_id + 1
                        move_weight = weights[el] * steps

                        best = min([move_weight + get_min(new_hall, new_rooms), best])

    visited_memo[memo_key] = best
    return visited_memo[memo_key]


# Example
# hall = {}
# rooms = [['B','A'], ['C', 'D'], ['B', 'C'], ['D', 'A']]

# print(get_min(hall, rooms))

# Example 2

# hall = {}
# rooms = [['B','D','D','A'], ['C', 'C', 'B', 'D'], ['B', 'B', 'A', 'C'], ['D', 'A', 'C','A']]

# print(get_min(hall, rooms))

# First Star

visited_memo={}
hall = {}
rooms = [['D','D'], ['A', 'A'], ['C', 'B'], ['C', 'B']]

print(get_min(hall, rooms))

# Second Star

visited_memo={}
hall = {}
rooms = [['D','D','D','D'], ['A','C','B','A'], ['C','B','A','B'], ['C','A','C','B']]

print(get_min(hall, rooms))