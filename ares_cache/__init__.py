"""ARES Cache
========
This module provides classes for managing and interacting with ARES data.

Optimized for:
- Memory efficiency using __slots__
- Performance with cached properties
- Clean API with minimal methods and essential dunder methods
"""

from .team import Team
from .alliance import Alliance
from .match import Match
from .event import Event
from .season import Season, History

__all__ = [
    "Team",
    "Alliance", 
    "Match",
    "Event",
    "Season",
    "History"
]

# Version information
__version__ = "2.0.0"