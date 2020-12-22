use std::collections::VecDeque;
use std::cmp::Ordering;

use anyhow::Result;
use aoclib::strtools;

pub fn part1(input: String) -> Result<usize> {
    let mut game = Combat::from_input(input.as_str())?;

    while !game.is_over() {
        game.play_round();
    }

    let w = game.winners_cards();

    Ok(
        w.into_iter().rev().enumerate()
            .map(|(i, c)| c * (i + 1))
            .sum()
    )
}

pub fn part2(input: String) -> Result<usize> {
    let game = Combat::from_input(input.as_str())?.recursive();

    let (game, _) = game.play_to_end();

    let w = game.0.winners_cards();

    Ok(
        w.into_iter().rev().enumerate()
            .map(|(i, c)| c * (i + 1))
            .sum()
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Combat {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

impl Combat {
    fn from_input(input: &str) -> Result<Self> {
        let (p1, p2) = strtools::split_once(input.trim(), "\n\n");
        let p1: VecDeque<usize> = p1.trim().lines().skip(1)
            .map(|l| l.parse::<usize>())
            .collect::<Result<_, _>>()?;
        let p2: VecDeque<usize> = p2.trim().lines().skip(1)
            .map(|l| l.parse::<usize>())
            .collect::<Result<_, _>>()?;
        Ok(
            Combat {
                player1: p1,
                player2: p2,
            }
        )
    }

    fn recursive(self) -> RecursiveCombat {
        RecursiveCombat(self)
    }

    fn is_over(&self) -> bool {
        self.player1.is_empty() || self.player2.is_empty()
    }

    fn play_round(&mut self) {
        let p1 = self.player1.pop_front().expect("game is over");
        let p2 = self.player2.pop_front().expect("game is over");

        match p1.cmp(&p2) {
            Ordering::Greater => {
                self.player1.push_back(p1);
                self.player1.push_back(p2);
            },
            Ordering::Less => {
                self.player2.push_back(p2);
                self.player2.push_back(p1);
            },
            _ => panic!("no round winner"),
        }
    }

    fn winners_cards(&self) -> Vec<usize> {
        let cards = match self.winner() {
            Winner::PlayerOne => &self.player1,
            Winner::PlayerTwo => &self.player2,
        };
        cards.iter().cloned().collect()
    }

    fn winner(&self) -> Winner {
        if self.player1.is_empty() {
            Winner::PlayerTwo
        } else if self.player2.is_empty() {
            Winner::PlayerOne
        } else {
            panic!("game is not over!")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecursiveCombat(Combat);

enum Winner {
    PlayerOne,
    PlayerTwo,
}

impl RecursiveCombat {
    fn play_to_end(self) -> (Self, Winner) {
        let mut past_rounds = Vec::new();

        let mut game = self;
        while !game.0.is_over() {
            if past_rounds.contains(&game) {
                return (game, Winner::PlayerOne);
            }
            past_rounds.push(game.clone());

            game.play_round();
        }

        let winner = game.0.winner();

        (game, winner)
    }

    fn _round_winner(&self, p1: usize, p2: usize) -> Winner {
        let subgame_p1: VecDeque<usize> = self.0.player1.iter().cloned().take(p1).collect();
        let subgame_p2: VecDeque<usize> = self.0.player2.iter().cloned().take(p2).collect();
        if subgame_p1.len() == p1 && subgame_p2.len() == p2 {
            let subgame = Self(
                Combat {
                    player1: subgame_p1,
                    player2: subgame_p2,
                }
            );

            let (_, winner) = subgame.play_to_end();
            winner
        } else {
            match p1.cmp(&p2) {
                Ordering::Greater => Winner::PlayerOne,
                Ordering::Less => Winner::PlayerTwo,
                _ => panic!("no round winner"),
            }
        }
    }

    fn play_round(&mut self) {
        let p1 = self.0.player1.pop_front().expect("game is over");
        let p2 = self.0.player2.pop_front().expect("game is over");

        match self._round_winner(p1, p2) {
            Winner::PlayerOne => {
                self.0.player1.push_back(p1);
                self.0.player1.push_back(p2);
            },
            Winner::PlayerTwo => {
                self.0.player2.push_back(p2);
                self.0.player2.push_back(p1);
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";

    #[test]
    fn part1_example() {
        assert_eq!(306, part1(EXAMPLE.to_string()).unwrap());
    }

    #[test]
    fn part2_example() {
        assert_eq!(291, part2(EXAMPLE.to_string()).unwrap());
    }
}
