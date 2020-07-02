import sys
sys.path.append('.')

from analysis.absearch import abSolver, Successor
import random

# %%


def gen_simple_state_graph():
    a = [str(chr(ord('a')+i)) for i in range(26)]
    a = ''.join(a)
    m = {}
    while(len(a)):
        c = a[0]
        a = a.replace(c, '')
        k = min(len(a), random.randint(0, 7))
        m[c] = random.sample(a, k=k) if k > 0 else random.randint(-10, 10)
    return m

# %%
class simple_successor(Successor):
    succ_map = {
        'a': ['e', 'z', 'h', 'x', 'k'],
        'b': ['c', 'h', 'k', 'l'],
        'c': ['z', 'o'],
        'd': ['u', 'o', 'y'],
        'e': ['w', 's', 'v', 'm'],
        'f': ['z', 'g', 'i', 'm'],
        'g': ['o', 'i', 'n'],
        'h': ['m', 'q', 'p', 's', 'l'],
        'i': ['y', 'p'],
        'j': ['v'],
        'k': -4,
        'l': 10,
        'm': ['q'],
        'n': 1,
        'o': ['t', 'p', 'q', 's'],
        'p': ['w', 'x', 'v', 't'],
        'q': ['u', 'y', 'w', 's', 'r', 'v'],
        'r': ['x'],
        's': -10,
        't': ['u', 'w', 'v', 'y', 'z'],
        'u': ['v'],
        'v': ['z', 'y', 'x', 'w'],
        'w': ['x'],
        'x': ['z', 'y'],
        'y': ['z'],
        'z': -8
    }

    def next(self, state, depth):
        return self.succ_map.get(state, 0)

    def is_term(self, state, depth):
        return isinstance(self.succ_map.get(state, 0), (int, float))


def test_absolver():
    successor = simple_successor()
    solver = abSolver(successor)
    value, _ = solver.maxmin('a')
    return value

if __name__ == "__main__":
    print(test_absolver())
