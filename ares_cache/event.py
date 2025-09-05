from dataclasses import dataclass, field
from typing import Dict, List
from .team import Team
from .match import Match

@dataclass
class Event:
    """Represents a robotics event with optimized team and match management."""
    eventCode: str = ""
    teams: List[Team] = field(default_factory=list)
    matches: Dict[str, Match] = field(default_factory=dict)

    def __str__(self):
        """String representation of the event."""
        return f"Event {self.eventCode}: {len(self.teams)} teams, {len(self.matches)} matches"

    def __hash__(self):
        """Make Event hashable based on event code."""
        return hash(self.eventCode)

    def __eq__(self, other):
        """Events are equal if they have the same event code."""
        if isinstance(other, Event):
            return self.eventCode == other.eventCode
        return False

    def __repr__(self):
        return f"Event(eventCode='{self.eventCode}', teams={len(self.teams)}, matches={len(self.matches)})"
