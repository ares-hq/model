from abc import ABC
from dataclasses import dataclass

@dataclass
class Scoring(ABC):
    score: float
    rank: int

    def __add__(self, other):
        if not isinstance(other, Scoring):
            return NotImplemented
        return self.__class__(self.score + other.score, None)
    
    def __str__(self):
        return f"{self.__class__.__name__} Score: {self.score:.2f} (Rank: {self.rank})" # TODO: Implement suffix for rank
    
    def __repr__(self):
        return f"{self.__class__.__name__}(score={self.score}, rank={self.rank})"
    
    def __eq__(self, value):
        return self.score == value

class Auto(Scoring):
    pass

class Teleop(Scoring):
    pass

class Endgame(Scoring):
    pass

class Penalties(Scoring):
    pass
