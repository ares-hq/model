from datetime import datetime
from ares_cache import Team
from ares_cache.scoring import Auto, Teleop, Endgame, Penalties # Hidden

team = Team(
    number=1234,
    name="Test Team",
    sponsors="Test Sponsor",
    location="Test Location",
    auto=Auto(score=100, rank=1),
    teleop=Teleop(score=200, rank=2),
    endgame=Endgame(score=50, rank=3),
    penalties=Penalties(score=10, rank=4),
    lastMatch=datetime.now(),
    lastChecked=datetime.now()
)

team2 = Team(
    number=5678,
    name="Another Team",
    lastMatch=datetime.now(),
    lastChecked=datetime.now()
)

def test_attributes():
    assert team.number == 1234
    assert team.name == "Test Team"
    assert team.sponsors == "Test Sponsor"
    assert team.location == "Test Location"
    assert team.auto.score == 100
    assert team.auto.rank == 1
    assert team.teleop.score == 200
    assert team.teleop.rank == 2
    assert team.endgame.score == 50
    assert team.endgame.rank == 3
    assert team.penalties.score == 10
    assert team.penalties.rank == 4
    assert isinstance(team.lastMatch, datetime)
    assert isinstance(team.lastChecked, datetime)

    # TEAM 2

    assert team2.number == 5678
    assert team2.name == "Another Team"
    assert isinstance(team2.lastMatch, datetime)
    assert isinstance(team2.lastChecked, datetime)

    assert isinstance(team2.auto, Auto)
    assert isinstance(team2.teleop, Teleop)
    assert isinstance(team2.endgame, Endgame)
    assert isinstance(team2.penalties, Penalties)

    assert team2.auto == 0
    assert team2.teleop == 0
    assert team2.endgame == 0
    assert team2.penalties == 0

def test_dict():
    d = team.__dict__
    assert d['number'] == 1234
    assert d['name'] == "Test Team"
    assert d['sponsors'] == "Test Sponsor"
    assert d['location'] == "Test Location"
    assert d['auto'].score == 100
    assert d['teleop'].score == 200
    assert d['endgame'].score == 50
    assert d['penalties'].score == 10
    assert isinstance(d['lastMatch'], datetime)
    assert isinstance(d['lastChecked'], datetime)


    for attr in team.__dict__:
        assert hasattr(team, attr), f"Team object does not have attribute {attr}"

    for attr in team2.__dict__:
        assert hasattr(team2, attr), f"Team object does not have attribute {attr}"

def test_str():
    s = str(team)
    assert "Number: 1234" in s
    assert "Name: Test Team" in s
    assert "Sponsors: Test Sponsor" in s
    assert "Location: Test Location" in s
    assert "Auto: 100.00 (Rank: 1)" in s
    assert "TeleOp: 200.00 (Rank: 2)" in s
    assert "Endgame: 50.00 (Rank: 3)" in s
    assert "Overall: 350.00 (Rank: 1)" in s or "Overall: 350.00" in s
    assert "Penalties: 10.00 (Rank: 4)" in s
    assert "Last Match: " in s
    assert "Last Checked: " in s