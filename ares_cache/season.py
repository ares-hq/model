from dataclasses import dataclass,field
from typing import Dict
from .event import Event, Team

@dataclass
class Season:
    """Stats for Season"""
    code: str = field(default_factory=str)
    """The code for the season, e.g., '2024'."""
    numTeams: int = field(default_factory=int)
    """Total number of teams in the season."""
    numAwards: int = field(default_factory=int)
    """Total number of awards in the season."""
    events: Dict[str, Event] = field(default_factory=dict)
    """Dictionary of all events in the season, keyed by event code."""
    teams: Dict[int, Team] = field(default_factory=dict)
    """Dictionary of all teams in the season, keyed by team number."""

@dataclass
class History:
    Seasons: Dict[str,Season] = field(default_factory=dict)

    def __post_init__(self):
        self.numSeasons = len(self.Seasons)