from dataclasses import dataclass, field
from .team import Team
from typing import List, Optional
from functools import cached_property

@dataclass
class Alliance:
    """Represents an alliance of two teams with optimized caching."""
    team1: Optional[Team] = None
    team2: Optional[Team] = None
    color: str = ""

    @cached_property
    def auto(self) -> float:
        """Combined autonomous score for the alliance."""
        score = 0.0
        if self.team1:
            score += self.team1.auto.score
        if self.team2:
            score += self.team2.auto.score
        return score

    @cached_property
    def teleop(self) -> float:
        """Combined teleop score for the alliance."""
        score = 0.0
        if self.team1:
            score += self.team1.teleop.score
        if self.team2:
            score += self.team2.teleop.score
        return score

    @cached_property
    def endgame(self) -> float:
        """Combined endgame score for the alliance."""
        score = 0.0
        if self.team1:
            score += self.team1.endgame.score
        if self.team2:
            score += self.team2.endgame.score
        return score

    @cached_property
    def penalties(self) -> float:
        """Combined penalties for the alliance."""
        score = 0.0
        if self.team1:
            score += self.team1.penalties.score
        if self.team2:
            score += self.team2.penalties.score
        return score

    @cached_property
    def overall(self) -> float:
        """Combined overall score for the alliance."""
        return self.auto + self.teleop + self.endgame

    @cached_property
    def numbers(self) -> List[int]:
        """Team numbers in the alliance."""
        return [
            self.team1.number if self.team1 else 0,
            self.team2.number if self.team2 else 0
        ]

    @cached_property
    def names(self) -> List[str]:
        """Team names in the alliance."""
        return [
            self.team1.name if self.team1 else "",
            self.team2.name if self.team2 else ""
        ]

    @cached_property
    def scoreboard(self) -> List[str]:
        """Formatted scoreboard data for the alliance."""
        return [
            str(self.numbers[0]),
            str(self.numbers[1]),
            f"{self.auto:.0f}",
            f"{self.teleop:.0f}",
            f"{self.endgame:.0f}",
            f"{self.penalties:.0f}",
            f"{self.overall:.0f}"
        ]

    @property
    def is_empty(self) -> bool:
        """Check if the alliance has meaningful data."""
        return all(score == 0.0 for score in [self.auto, self.teleop, self.endgame, self.penalties, self.overall])

    def __hash__(self):
        """Make Alliance hashable based on team numbers."""
        return hash((self.numbers[0], self.numbers[1], self.color))

    def __eq__(self, other):
        """Alliances are equal if they have the same teams and color."""
        if isinstance(other, Alliance):
            return (self.numbers == other.numbers and self.color == other.color)
        return False

    def __repr__(self):
        return f"Alliance(team1={self.team1.number if self.team1 else None}, team2={self.team2.number if self.team2 else None}, color='{self.color}')"