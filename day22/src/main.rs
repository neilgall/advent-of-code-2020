use std::collections::VecDeque;
use parser::*;

// -- model

type Card = i64;
type Score = i64;
type PlayerID = usize;
type Player = VecDeque<Card>;

#[derive(Debug, PartialEq)]
struct Game {
    players: Vec<Player>
}

impl Game {
    fn cards(&self, player: PlayerID) -> Vec<Card> {
        self.players[player].iter().copied().collect()
    }

    fn play_round(&mut self) {
        let mut top_cards: Vec<(PlayerID, Card)> =
            self.players.iter_mut().filter_map(|p| p.pop_front()).enumerate().collect();

        let winner = top_cards.iter().max_by_key(|(_, card)| card).unwrap().0;

        top_cards.sort_by(|(_, card1), (_, card2)| card2.cmp(card1));
        for (_, card) in top_cards {
            self.players[winner].push_back(card);
        }
    }

    fn over(&self) -> bool {
        self.players.iter().any(|p| p.is_empty())
    }

    fn winning_score(&self) -> Score {
        let winner: &Player = self.players.iter().filter(|p| !p.is_empty()).next().unwrap();
        winner.iter().rev().enumerate().fold(
            0,
            |score, (index, card)| score + card * ((index+1) as Score)
        )
    }
}

// -- parser

fn parse_input(input: &str) -> ParseResult<Game> {
    let player_tag = integer.between(match_literal("Player "), match_literal(":"));
    let cards = one_or_more(whitespace_wrap(integer)).map(|cards| cards.into_iter().collect());
    let player = right(player_tag, cards);
    let game = one_or_more(player).map(|players| Game { players });
    game.parse(input)
}

// -- problems

fn part1(game: &mut Game) -> Score {
    while !game.over() {
        game.play_round();   
    }
    game.winning_score()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut game = parse_input(&input).unwrap().1;
    println!("part 1 {}", part1(&mut game));
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_game() -> Game {
        Game {
            players: vec![
                vec![9, 2, 6, 3, 1].into_iter().collect(),
                vec![5, 8, 4, 7, 10].into_iter().collect()
            ]
        }
    }

    #[test]
    fn test_parser() {
        let game = parse_input(
            "Player 1:
             9
             2
             6
             3
             1

             Player 2:
             5
             8
             4
             7
             10");
        assert_eq!(game, Ok(("", test_game())));
    }

    #[test]
    fn test_play_round() {
        let mut game = test_game();
        game.play_round();
        assert_eq!(game.cards(0), vec![2, 6, 3, 1, 9, 5]);
        assert_eq!(game.cards(1), vec![8, 4, 7, 10]);

        game.play_round();
        assert_eq!(game.cards(0), vec![6, 3, 1, 9, 5]);
        assert_eq!(game.cards(1), vec![4, 7, 10, 8, 2]);

        game.play_round();
        assert_eq!(game.cards(0), vec![3, 1, 9, 5, 6, 4]);
        assert_eq!(game.cards(1), vec![7, 10, 8, 2]);
    }

    #[test]
    fn test_game_over() {
        let mut game = test_game();
        for _ in 0..29 {
            assert!(!game.over());
            game.play_round();
        }
        assert!(game.over());
    }

    #[test]
    fn test_score() {
        let mut game = test_game();
        while !game.over() { game.play_round(); }
        assert_eq!(game.winning_score(), 306);
    }
}
