from .alliance import Alliance
from dataclasses import dataclass, field

@dataclass
class Match:
    redAlliance: Alliance
    blueAlliance: Alliance
    winner: str = field(default_factory=str)

    def __post_init__(self):
        self.matchCategories = [
            "Team 1", "Team 2", "Autonomous", "TeleOp", "End Game", "Penalties", "Final Score"
        ]

        self.redAllianceEmpty = all(
            value in [0, "", None] for value in [
                self.redAlliance.combined_autoOPR,
                self.redAlliance.combined_teleOPR,
                self.redAlliance.combined_endgameOPR,
                self.redAlliance.combined_penalties,
                self.redAlliance.combined_overallOPR
            ]
        )
        self.blueAllianceEmpty = all(
            value in [0, "", None] for value in [
                self.blueAlliance.combined_autoOPR,
                self.blueAlliance.combined_teleOPR,
                self.blueAlliance.combined_endgameOPR,
                self.blueAlliance.combined_penalties,
                self.blueAlliance.combined_overallOPR
            ]
        )

        if not self.redAllianceEmpty and not self.blueAllianceEmpty and not self.winner:
            if self.redAlliance.combined_overallOPR > self.blueAlliance.combined_overallOPR:
                self.winner = self.redAlliance.color
            elif self.redAlliance.combined_overallOPR < self.blueAlliance.combined_overallOPR:
                self.winner = self.blueAlliance.color
            else:
                self.winner = "Tie"

    def __str__(self):
        """ASCII Table Representation of Match Object."""
        table = "\rCategory      | Red Alliance | Blue Alliance\n"
        table += "--------------|--------------|--------------\n"
        for category, red_score, blue_score in zip(
            self.matchCategories, self.redAlliance.scoreboard, self.blueAlliance.scoreboard
        ):
            table += f"{category:<14}| {red_score:<13}| {blue_score:<13}\n"
        if self.winner:
            table += f"Match Winner: {self.winner}\n"
        return table