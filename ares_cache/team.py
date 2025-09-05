from dataclasses import dataclass, field
from .scoring import Auto, Teleop, Endgame, Penalties
from datetime import datetime
from functools import cached_property

@dataclass
class Team:
    """
    Represents a robotics team with both performance statistics and profile information.
    Optimized for memory efficiency and performance.
    """
    # Metadata
    lastMatch: datetime
    lastChecked: datetime
    
    # Team profile information    
    number: int
    name: str
    sponsors: str = ""
    location: str = ""

    auto: Auto = field(default_factory=Auto)
    teleop: Teleop = field(default_factory=Teleop)
    endgame: Endgame = field(default_factory=Endgame)
    penalties: Penalties = field(default_factory=Penalties)

    @cached_property
    def overall(self) -> Auto:
        """Calculate overall OPR with caching."""
        result = self.auto + self.teleop + self.endgame
        # Calculate overall rank if individual ranks are available
        if all(getattr(score, 'rank', 0) > 0 for score in [self.auto, self.teleop, self.endgame]):
            # Simple ranking based on average of component ranks
            avg_rank = (self.auto.rank + self.teleop.rank + self.endgame.rank) // 3
            result.rank = avg_rank
        return result

    def __str__(self):
        """String representation of the team."""
        return (
            f"Number: {self.number}\n"
            f"Name: {self.name}\n"
            f"Sponsors: {self.sponsors}\n"
            f"Location: {self.location}\n"
            f"Auto: {self.auto.score:.2f} (Rank: {self.auto.rank})\n"
            f"TeleOp: {self.teleop.score:.2f} (Rank: {self.teleop.rank})\n"
            f"Endgame: {self.endgame.score:.2f} (Rank: {self.endgame.rank})\n"
            f"Overall: {self.overall.score:.2f} (Rank: {self.overall.rank})\n"
            f"Penalties: {self.penalties.score:.2f} (Rank: {self.penalties.rank})\n"
            f"Last Match: {self.lastMatch}\n"
            f"Last Checked: {self.lastChecked}"
        )

    def __hash__(self):
        """Make Team hashable for use in sets and as dict keys."""
        return hash(self.number)

    def __eq__(self, other):
        """Teams are equal if they have the same number."""
        if isinstance(other, Team):
            return self.number == other.number
        return False

    def __repr__(self):
        return f"Team(number={self.number}, name='{self.name}')"