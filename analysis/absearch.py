import abc
import math


class Successor(metaclass=abc.ABCMeta):
    @abc.abstractclassmethod
    def next(self, state, depth):
        pass

    @abc.abstractclassmethod
    def is_term(self, state, depth) -> bool:
        pass

class abSolver():
    def __init__(self, successor: Successor):
        super().__init__()
        self.successor = successor
        self.depth = 0

    def minmax(self, state):
        return self.__min__(state, -math.inf, math.inf)

    def maxmin(self, state):
        return self.__max__(state, -math.inf, math.inf)

    def __min__(self, state, a, b, depth=0):
        v = math.inf
        depth += 1
        print(f'min {state}, a:{a}, b:{b}, depth:{depth}')
        if self.successor.is_term(state, depth):
            return self.successor.next(state, depth), True
        for next_state in self.successor.next(state, depth):
            _v, from_term = self.__max__(next_state, a, b, depth)
            v = min(v, _v)
            if v <= a:
                if not from_term:
                    print(f'min {state} pruned: v:{v}, a:{a}')
                return v, False
            b = min(b, v)
        print(f'min {state}, a:{a}, b:{b}, depth:{depth}, v:{v}')
        return v, False

    def __max__(self, state, a, b, depth=0):
        v = -math.inf
        depth += 1
        print(f'max {state}, a:{a}, b:{b}, depth:{depth}')
        if self.successor.is_term(state, depth):
            print(f'terminal state: {state}')
            return self.successor.next(state, depth), True
        for next_state in self.successor.next(state, depth):
            _v, from_term = self.__min__(next_state, a, b, depth)
            v = max(v, _v)
            if v >= b:
                if not from_term:
                    print(f'max {state} pruned: v:{v}, b:{b}')
                return v, False
            a = max(a, v)
        print(f'max {state}, a:{a}, b:{b}, depth:{depth}, v:{v}')
        return v, False