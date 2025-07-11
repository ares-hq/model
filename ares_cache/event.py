from dataclasses import dataclass, field
from typing import Dict, List
from .team import Team
from .match import Match

@dataclass
class Event:
    eventCode: str = field(default_factory=str)
    teams: List[Team] = field(default_factory=list)
    matches: Dict[str, Match] = field(default_factory=dict)
