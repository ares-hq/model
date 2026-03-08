use cache::scoring::{Auto, Teleop, Endgame, Penalties};

#[test]
fn test_auto_default() {
    let auto = Auto::default();
    assert_eq!(auto.score, 0);
    assert_eq!(auto.rank, 0);
}

#[test]
fn test_auto_add() {
    let auto1 = Auto { score: 100, rank: 1 };
    let auto2 = Auto { score: 50, rank: 2 };
    let result = auto1 + auto2;
    assert_eq!(result.score, 150);
    assert_eq!(result.rank, 0); // rank is reset to 0 after addition
}

#[test]
fn test_auto_sub() {
    let auto1 = Auto { score: 100, rank: 1 };
    let auto2 = Auto { score: 30, rank: 2 };
    let result = auto1 - auto2;
    assert_eq!(result.score, 70);
    assert_eq!(result.rank, 0); // rank is reset to 0 after subtraction
}

#[test]
fn test_teleop_default() {
    let teleop = Teleop::default();
    assert_eq!(teleop.score, 0);
    assert_eq!(teleop.rank, 0);
}

#[test]
fn test_teleop_add() {
    let teleop1 = Teleop { score: 200, rank: 1 };
    let teleop2 = Teleop { score: 100, rank: 2 };
    let result = teleop1 + teleop2;
    assert_eq!(result.score, 300);
    assert_eq!(result.rank, 0);
}

#[test]
fn test_teleop_sub() {
    let teleop1 = Teleop { score: 200, rank: 1 };
    let teleop2 = Teleop { score: 75, rank: 2 };
    let result = teleop1 - teleop2;
    assert_eq!(result.score, 125);
    assert_eq!(result.rank, 0);
}

#[test]
fn test_endgame_default() {
    let endgame = Endgame::default();
    assert_eq!(endgame.score, 0);
    assert_eq!(endgame.rank, 0);
}

#[test]
fn test_endgame_add() {
    let endgame1 = Endgame { score: 50, rank: 1 };
    let endgame2 = Endgame { score: 30, rank: 2 };
    let result = endgame1 + endgame2;
    assert_eq!(result.score, 80);
    assert_eq!(result.rank, 0);
}

#[test]
fn test_endgame_sub() {
    let endgame1 = Endgame { score: 50, rank: 1 };
    let endgame2 = Endgame { score: 20, rank: 2 };
    let result = endgame1 - endgame2;
    assert_eq!(result.score, 30);
    assert_eq!(result.rank, 0);
}

#[test]
fn test_penalties_default() {
    let penalties = Penalties::default();
    assert_eq!(penalties.score, 0);
    assert_eq!(penalties.rank, 0);
}

#[test]
fn test_penalties_add() {
    let penalties1 = Penalties { score: 10, rank: 1 };
    let penalties2 = Penalties { score: 20, rank: 2 };
    let result = penalties1 + penalties2;
    assert_eq!(result.score, 30);
    assert_eq!(result.rank, 0);
}

#[test]
fn test_penalties_sub() {
    let penalties1 = Penalties { score: 30, rank: 1 };
    let penalties2 = Penalties { score: 10, rank: 2 };
    let result = penalties1 - penalties2;
    assert_eq!(result.score, 20);
    assert_eq!(result.rank, 0);
}

#[test]
fn test_scoring_chain_operations() {
    let auto1 = Auto { score: 100, rank: 1 };
    let auto2 = Auto { score: 50, rank: 2 };
    let auto3 = Auto { score: 25, rank: 3 };
    
    let result = auto1 + auto2 + auto3;
    assert_eq!(result.score, 175);
}

#[test]
fn test_zero_scores() {
    let auto1 = Auto { score: 0, rank: 0 };
    let auto2 = Auto { score: 0, rank: 0 };
    let result = auto1 + auto2;
    assert_eq!(result.score, 0);
}

#[test]
fn test_max_realistic_scores() {
    // Test with realistic maximum FTC scores
    let auto = Auto { score: 150, rank: 1 };
    let teleop = Teleop { score: 200, rank: 1 };
    let endgame = Endgame { score: 60, rank: 1 };
    
    assert_eq!(auto.score, 150);
    assert_eq!(teleop.score, 200);
    assert_eq!(endgame.score, 60);
}
