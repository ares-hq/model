from .alliance import Alliance
from dataclasses import dataclass, field
from functools import cached_property
from typing import Literal

WinnerType = Literal["red", "blue", "tie", ""]

@dataclass
class Match:
    """Represents a match between two alliances."""
    redAlliance: Alliance
    blueAlliance: Alliance
    _winner: WinnerType = field(default="", init=False)

    @cached_property
    def matchCategories(self) -> list[str]:
        """Categories for match display."""
        return ["Team 1", "Team 2", "Autonomous", "TeleOp", "End Game", "Penalties", "Final Score"]

    @property
    def winner(self) -> WinnerType:
        """Determine the winner of the match."""
        if self._winner:  # If manually set
            return self._winner
        elif self.redAlliance.is_empty and self.blueAlliance.is_empty:
            return ""
        else:
            red_score = self.redAlliance.overall
            blue_score = self.blueAlliance.overall
            
            if red_score > blue_score:
                return "red"
            elif blue_score > red_score:
                return "blue"
            else:
                return "tie"

    @winner.setter
    def winner(self, value: WinnerType):
        """Set the winner manually."""
        object.__setattr__(self, '_winner', value)

    @property
    def redAllianceEmpty(self) -> bool:
        """Check if red alliance has no meaningful data."""
        return self.redAlliance.is_empty

    @property
    def blueAllianceEmpty(self) -> bool:
        """Check if blue alliance has no meaningful data."""
        return self.blueAlliance.is_empty

    def __str__(self):
        """ASCII table representation of the match."""
        header = "Category      | Red Alliance | Blue Alliance\n"
        separator = "--------------|--------------|--------------\n"
        
        rows = []
        for category, red_score, blue_score in zip(
            self.matchCategories, 
            self.redAlliance.scoreboard, 
            self.blueAlliance.scoreboard
        ):
            rows.append(f"{category:<14}| {red_score:<13}| {blue_score:<13}\n")
        
        table_parts = [header, separator] + rows
        if self.winner:
            table_parts.append(f"Match Winner: {self.winner.title()}\n")
        
        return "".join(table_parts)

    def __hash__(self):
        """Make Match hashable based on alliance composition."""
        return hash((
            tuple(self.redAlliance.numbers),
            tuple(self.blueAlliance.numbers),
            self.redAlliance.color,
            self.blueAlliance.color
        ))

    def __eq__(self, other):
        """Matches are equal if they have the same alliances."""
        if isinstance(other, Match):
            return (self.redAlliance == other.redAlliance and 
                   self.blueAlliance == other.blueAlliance)
        return False

    def __repr__(self):
        return f"Match(red={self.redAlliance.numbers}, blue={self.blueAlliance.numbers})"