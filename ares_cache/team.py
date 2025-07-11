from dataclasses import dataclass, field
from .scoring import Auto, Teleop, Endgame, Penalties
from datetime import datetime

@dataclass
class Team:
    """
    Represents a robotics team with both performance statistics and profile information.

    Attributes:
        teamNumber (int): The team number.
        teamName (str): The name of the team.
        sponsors (str): The sponsors of the team.
        location (str): The location of the team.
    """
    # Metadata
    lastMatch: datetime
    lastChecked: datetime
    
    # Team profile information    
    number: int
    name: str
    sponsors: str = field(default_factory=str)
    location: str = field(default_factory=str)

    auto: Auto = field(default_factory=Auto)
    teleop: Teleop = field(default_factory=Teleop)
    endgame: Endgame = field(default_factory=Endgame)
    penalties: Penalties = field(default_factory=Penalties)

    def __post_init__(self):
        """Calculate overall OPR if not provided."""
        self.overall = self.auto + self.teleop + self.endgame

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

    # def update_profile(self, name: str = None, sponsors: str = None, location: str = None, timestamp: str = None):
    #     """
    #     Update the team's profile information.

    #     Args:
    #         name (str): New team name.
    #         sponsors (str): Updated sponsors.
    #         location (str): Updated location.
    #         timestamp (str): Timestamp of the update.
    #     """
    #     if name:
    #         self.name = name
    #     if sponsors:
    #         self.sponsors = sponsors
    #     if location:
    #         self.location = location
    #     if timestamp:
    #         self.lastChecked = timestamp

    # def add_stats(self, autoOPR: float = 0, teleOPR: float = 0, endgameOPR: float = 0, penalties: float = 0):
    #     """
    #     Add stats to the team's performance.

    #     Args:
    #         autoOPR (float): Additional autonomous OPR.
    #         teleOPR (float): Additional teleoperated OPR.
    #         endgameOPR (float): Additional endgame OPR.
    #         penalties (float): Additional penalties.
    #     """
    #     self.auto += autoOPR
    #     self.teleop += teleOPR
    #     self.endgame += endgameOPR
    #     self.penalties += penalties
    #     self.overall = self.auto + self.teleop + self.endgame