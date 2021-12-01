import fileinput
import functools
import itertools
import re
import math
from collections import deque
import sys
import numpy as np
import copy

def simulate_game(player_1, player_2):
  while(player_1 and player_2):
    if player_1[0] > player_2[0]:
      p1_card = player_1.pop(0)
      p2_card = player_2.pop(0)
      player_1.append(p1_card)
      player_1.append(p2_card)
    else:
      p1_card = player_1.pop(0)
      p2_card = player_2.pop(0)
      player_2.append(p2_card)
      player_2.append(p1_card)

  if not player_2 : return ('p1', player_1)
  if not player_1 : return ('p2', player_2)

def simulate_rec_game(player_1, player_2, visited_1 = set(), visited_2 = set()):
  game_round = 1
  while player_1 and player_2:
    # print(f"Game #{game_round}: ", player_1, player_2)
    game_round += 1
    key1 = str(player_1)
    key2 = str(player_2)
    if key1 in visited_1 or key2 in visited_2:
      return ('p1', player_1)
    
    visited_1.add(key1)
    visited_2.add(key2)

    top_1 = player_1.pop(0) 
    top_2 = player_2.pop(0)

    p1_wins = False
    if top_1 <= len(player_1) and top_2 <= len(player_2):
      # call a new subgame with new sets (visited only is applied for game)
      sub_game = simulate_rec_game(player_1[:top_1], player_2[:top_2], set(), set())
      if sub_game[0] == 'p1' : p1_wins = True
    else:    
      if top_1 > top_2:
        p1_wins = True

    if p1_wins:
      player_1.append(top_1)
      player_1.append(top_2)
    else:
      player_2.append(top_2)
      player_2.append(top_1)
  
  if not player_2 : return ('p1', player_1)
  if not player_1 : return ('p2', player_2)
    

def get_score(cards):
  score = 0
  for idx in range(0, len(cards)):
    score += cards[idx] * (len(cards)-idx)
  return score

player_1 = []
player_2 = []
reading_1 = True
for line in fileinput.input():
  clean_line = line.strip()

  if "Player 2" in clean_line:
    reading_1 = False

  if reading_1 and clean_line.isnumeric():
    player_1.append(int(clean_line))

  if not reading_1 and clean_line.isnumeric():
    player_2.append(int(clean_line))

# First Star
winner, cards = simulate_game(player_1.copy(), player_2.copy())
print(get_score(cards))

# Second Star
winner, cards = simulate_rec_game(player_1, player_2)
print(get_score(cards))