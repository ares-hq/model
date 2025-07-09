from ares_cache.team import Team
import pytest
from datetime import datetime, timedelta
from unittest.mock import MagicMock
# Mock scoring classes
class MockScore:
    def __init__(self, score=0.0, rank=0):
        self.score = score
        self.rank = rank

    def __add__(self, other):
        if isinstance(other, (int, float)):
            return MockScore(self.score + other, self.rank)
        elif isinstance(other, MockScore):
            return MockScore(self.score + other.score, self.rank)
        return NotImplemented

    def __iadd__(self, other):
        if isinstance(other, (int, float)):
            self.score += other
        elif isinstance(other, MockScore):
            self.score += other.score
        return self

Auto = Teleop = Endgame = Penalties = MockScore

def make_team():
    return Team(
        number=1234,
        name="Test Team",
        sponsors="Sponsor A",
        location="Test City",
        auto=Auto(10, 1),
        teleop=Teleop(20, 2),
        endgame=Endgame(30, 3),
        penalties=Penalties(5, 4),
        eventDate=datetime(2023, 1, 1),
        lastUpdated=datetime(2023, 1, 2)
    )

def test_team_initialization(monkeypatch):
    monkeypatch.setattr("ares_cache.team.Auto", Auto)
    monkeypatch.setattr("ares_cache.team.Teleop", Teleop)
    monkeypatch.setattr("ares_cache.team.Endgame", Endgame)
    monkeypatch.setattr("ares_cache.team.Penalties", Penalties)
    team = make_team()
    assert team.number == 1234
    assert team.name == "Test Team"
    assert team.sponsors == "Sponsor A"
    assert team.location == "Test City"
    assert team.auto.score == 10
    assert team.teleop.score == 20
    assert team.endgame.score == 30
    assert team.penalties.score == 5
    assert team.eventDate == datetime(2023, 1, 1)
    assert team.lastUpdated == datetime(2023, 1, 2)
    assert team.overall.score == 60  # 10+20+30

def test_team_str(monkeypatch):
    monkeypatch.setattr("ares_cache.team.Auto", Auto)
    monkeypatch.setattr("ares_cache.team.Teleop", Teleop)
    monkeypatch.setattr("ares_cache.team.Endgame", Endgame)
    monkeypatch.setattr("ares_cache.team.Penalties", Penalties)
    team = make_team()
    s = str(team)
    assert "Team Number: 1234" in s
    assert "Name: Test Team" in s
    assert "Sponsors: Sponsor A" in s
    assert "Location: Test City" in s
    assert "Auto: 10.00 (Rank: 1)" in s
    assert "TeleOp: 20.00 (Rank: 2)" in s
    assert "Endgame: 30.00 (Rank: 3)" in s
    assert "Overall: 60.00 (Rank: 1)" in s
    assert "Penalties: 5.00 (Rank: 4)" in s

def test_update_profile(monkeypatch):
    monkeypatch.setattr("ares_cache.team.Auto", Auto)
    monkeypatch.setattr("ares_cache.team.Teleop", Teleop)
    monkeypatch.setattr("ares_cache.team.Endgame", Endgame)
    monkeypatch.setattr("ares_cache.team.Penalties", Penalties)
    team = make_team()
    new_time = datetime(2023, 2, 2)
    team.update_profile(name="New Name", sponsors="New Sponsor", location="New City", timestamp=new_time)
    assert team.name == "New Name"
    assert team.sponsors == "New Sponsor"
    assert team.location == "New City"
    assert team.lastUpdated == new_time

def test_add_stats(monkeypatch):
    monkeypatch.setattr("ares_cache.team.Auto", Auto)
    monkeypatch.setattr("ares_cache.team.Teleop", Teleop)
    monkeypatch.setattr("ares_cache.team.Endgame", Endgame)
    monkeypatch.setattr("ares_cache.team.Penalties", Penalties)
    team = make_team()
    team.add_stats(autoOPR=5, teleOPR=10, endgameOPR=15, penalties=2)
    assert team.auto.score == 15
    assert team.teleop.score == 30
    assert team.endgame.score == 45
    assert team.penalties.score == 7
    assert team.overall.score == 90  # 15+30+45

def test_update_profile_partial(monkeypatch):
    monkeypatch.setattr("ares_cache.team.Auto", Auto)
    monkeypatch.setattr("ares_cache.team.Teleop", Teleop)
    monkeypatch.setattr("ares_cache.team.Endgame", Endgame)
    monkeypatch.setattr("ares_cache.team.Penalties", Penalties)
    team = make_team()
    team.update_profile(name="Partial Name")
    assert team.name == "Partial Name"
    assert team.sponsors == "Sponsor A"
    assert team.location == "Test City"
