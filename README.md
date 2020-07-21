# Geistchess - Simple <strike>python</strike> rust+python chess engine.
## Tasks:
1. Board representation. ✔
2. Move generation (rewritten in rust). ✔
    1. Generate { piece, position -> attack mask } mappings for knight and king. ✔
    1. Algebraic notation in/out. (Very verbose AN ✔)
    2. Switch to magic bitboards for sliding pieces attacks. ✔
    3. Purely legal movegen. (almost, except enpassant capture leading to check along rank4) ✔
    4. FEN position input. ✔
    5. Perft test, Passed perft(4). ✔ *only total node count checked
    6. Profile movegen. **~**
3. UCI protocol support.
4. Move Analysis:
    1. Eval function. **~**
    2. Search
    3. Opening/Endgame tables
5. RL. Reinforcement learning!!?