# Geistchess - Simple <strike>python</strike> rust+python chess engine.
## Tasks:
1. Board representation. ✔
2. Move generation (rewritten in rust). ✔
    1. Generate { piece, position -> attack mask } mappings for knight and king. ✔
    1. Algebraic notation in/out. (Very verbose AN ✔) **~**
    2. Switch to magic bitboards for sliding pieces attacks. ✔
3. UCI protocol support.
4. Move Analysis:
    1. Search **~**
    2. Opening/Endgame tables
5. RL. Reinforcement learning!!?