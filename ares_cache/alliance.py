from dataclasses import dataclass, field
from .team import Team

@dataclass
class Alliance:
    team1: Team = field(default_factory=Team)
    team2: Team = field(default_factory=Team)
    color: str = field(default_factory=str)
    combined_autoOPR: float = 0.0
    combined_teleOPR: float = 0.0
    combined_endgameOPR: float = 0.0
    combined_penalties: float = 0.0
    combined_overallOPR: float = 0.0

    def __post_init__(self):
        if self.team1 and self.team2:
            self.combined_autoOPR = self.team1.auto + self.team2.auto
            self.combined_teleOPR = self.team1.teleop + self.team2.teleop
            self.combined_endgameOPR = self.team1.endgame + self.team2.endgame
            self.combined_penalties = self.team1.penalties + self.team2.penalties
            self.combined_overallOPR = self.combined_autoOPR + self.combined_teleOPR + self.combined_endgameOPR

            self.teamNums: List[int] = [self.team1.number, self.team2.number]
            self.teamNames: List[str] = [self.team1.name, self.team2.name]
        else: 
            self.teamNums: List[int] = [0, 0]
            self.teamNames: List[str] = ["", ""]

        self.scoreboard: List[str] = [
            f"{self.teamNums[0]}",
            f"{self.teamNums[1]}",
            f"{self.combined_autoOPR:.0f}",
            f"{self.combined_teleOPR:.0f}",
            f"{self.combined_endgameOPR:.0f}",
            f"{self.combined_penalties:.0f}",
            f"{self.combined_overallOPR:.0f}"
        ]