from abc import ABC, abstractmethod


class Scoring(ABC):
    @abstractmethod
    def __init__(self, score: float, rank: int):
        self.score = score
        self.rank = rank

    def __add__(self, other):
        if not isinstance(other, Scoring):
            return NotImplemented
        return self.__class__(self.score + other.score, min(self.rank, other.rank))

class Auto(Scoring):
    def __init__(self, score: float, rank: int):
        super().__init__(score, rank)

class Teleop(Scoring):
    def __init__(self, score: float, rank: int):
        super().__init__(score, rank)

class Endgame(Scoring):
    def __init__(self, score: float, rank: int):
        super().__init__(score, rank)

class Penalties(Scoring):
    def __init__(self, score: float, rank: int):
        super().__init__(score, rank)