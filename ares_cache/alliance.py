from dataclasses import dataclass, field
from .team import Team
from typing import List

@dataclass
class Alliance:
    team1: Team = field(default_factory=Team)
    team2: Team = field(default_factory=Team)
    color: str = field(default_factory=str)
    auto: float = 0.0
    teleop: float = 0.0
    endgame: float = 0.0
    penalties: float = 0.0
    overall: float = 0.0

    def __post_init__(self):
        if self.team1 and self.team2:
            self.auto = self.team1.auto.score + self.team2.auto.score
            self.teleop = self.team1.teleop.score + self.team2.teleop.score
            self.endgame = self.team1.endgame.score + self.team2.endgame.score
            self.penalties = self.team1.penalties.score + self.team2.penalties.score
            self.overall = self.auto + self.teleop + self.endgame

            self.numbers: List[int] = [self.team1.number, self.team2.number]
            self.names: List[str] = [self.team1.name, self.team2.name]
        else: 
            self.numbers: List[int] = [0, 0]
            self.names: List[str] = ["", ""]

        self.scoreboard: List[str] = [
            f"{self.numbers[0]}",
            f"{self.numbers[1]}",
            f"{self.auto:.0f}",
            f"{self.teleop:.0f}",
            f"{self.endgame:.0f}",
            f"{self.penalties:.0f}",
            f"{self.overall:.0f}"
        ]