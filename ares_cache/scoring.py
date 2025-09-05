from abc import ABC
from dataclasses import dataclass

@dataclass
class Scoring(ABC):
    score: float = 0.0
    rank: int = 0

    def __add__(self, other):
        if not isinstance(other, Scoring):
            return NotImplemented
        return self.__class__(self.score + other.score, 0)
    
    def __str__(self):
        suffix = ""
        if self.rank > 0:
            if 10 <= self.rank % 100 <= 20:
                suffix = "th"
            else:
                suffix_map = {1: "st", 2: "nd", 3: "rd"}
                suffix = suffix_map.get(self.rank % 10, "th")
        rank_str = f" (Rank: {self.rank}{suffix})" if self.rank > 0 else ""
        return f"{self.__class__.__name__} Score: {self.score:.2f}{rank_str}"
    
    def __repr__(self):
        return f"{self.__class__.__name__}(score={self.score}, rank={self.rank})"
    
    def __eq__(self, value):
        if isinstance(value, Scoring):
            return self.score == value.score and self.rank == value.rank
        return self.score == value
    
    def __hash__(self):
        return hash((self.score, self.rank))

class Auto(Scoring):
    pass

class Teleop(Scoring):
    pass

class Endgame(Scoring):
    pass

class Penalties(Scoring):
    pass
