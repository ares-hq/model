from dataclasses import dataclass, field
from typing import Dict
from .event import Event, Team

@dataclass
class Season:
    """Season statistics."""
    code: str = ""
    numTeams: int = 0
    numAwards: int = 0
    events: Dict[str, Event] = field(default_factory=dict)
    teams: Dict[int, Team] = field(default_factory=dict)

    def __str__(self):
        return f"Season {self.code}: {len(self.events)} events, {len(self.teams)} teams"

    def __hash__(self):
        return hash(self.code)

    def __eq__(self, other):
        if isinstance(other, Season):
            return self.code == other.code
        return False

    def __repr__(self):
        return f"Season(code='{self.code}', events={len(self.events)}, teams={len(self.teams)})"


@dataclass
class History:
    """History management for seasons."""
    Seasons: Dict[str, Season] = field(default_factory=dict)

    @property
    def numSeasons(self) -> int:
        return len(self.Seasons)

    def __str__(self):
        return f"History: {self.numSeasons} seasons"

    def __hash__(self):
        return hash(tuple(sorted(self.Seasons.keys())))

    def __eq__(self, other):
        if isinstance(other, History):
            return set(self.Seasons.keys()) == set(other.Seasons.keys())
        return False

    def __repr__(self):
        return f"History(seasons={list(self.Seasons.keys())})"