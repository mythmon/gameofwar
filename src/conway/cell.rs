use std::collections::HashMap;
use rand::{Rng, Rand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Red,
    Blue,
    Neutral,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub alive: bool,
    pub team: Team,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            alive: false,
            team: Team::Neutral,
        }
    }

    pub fn inherit_from(&mut self, parents: &[&Cell]) {
        let mut counts: HashMap<Team, u8> = HashMap::new();
        counts.insert(Team::Red, 0);
        counts.insert(Team::Blue, 0);
        counts.insert(Team::Neutral, 0);

        for p in parents {
            *counts.entry(p.team).or_insert(0) += 1;
        }

        self.team = match (counts[&Team::Red], counts[&Team::Blue], counts[&Team::Neutral]) {
            (r, b, _) if r >= 1 && b == 0 => Team::Red,
            (r, b, _) if b >= 1 && r == 0 => Team::Blue,
            _ => Team::Neutral,
        };
    }
}

impl Rand for Cell {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Cell {
            alive: rng.gen::<f32>() < 0.4,
            team: *rng.choose(&[Team::Red, Team::Blue]).unwrap(),
        }
    }
}
