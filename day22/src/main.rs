use log::debug;
use std::collections::{HashSet, HashMap, VecDeque};
use parser::*;

// -- model

type Card = i64;
type Score = i64;
type PlayerID = usize;
type Player = VecDeque<Card>;
type GameState = Vec<Player>;
type GameMemos = HashMap<Player, PlayerID>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Rules {
    Normal,
    Recursive
}

#[derive(Debug, PartialEq)]
struct Game {
    players: GameState,
    history: HashSet<Player>,
    prefix: String,
    round: usize
}

impl Game {
    fn new(players: GameState) -> Self {
        Game {
            players,
            history: HashSet::new(),
            prefix: "".to_string(),
            round: 0
        }
    }

    fn make_sub_game(&self, drawn: &Vec<(PlayerID, Card)>) -> Game {
        let mut game = Game::new(
            drawn.iter().map(|(player, card)|
                self.players[*player].iter().take(*card as usize).copied().collect()
            ).collect()
        );
        game.prefix = format!("{}  ", self.prefix);
        game
    }

    fn cards(&self, player: PlayerID) -> Vec<Card> {
        self.players[player].iter().copied().collect()
    }

    fn should_recurse(&self, cards: &Vec<(PlayerID, Card)>) -> bool {
        cards.iter().all(|(player, card)| self.players[*player].len() >= *card as usize)
    }

    fn play_round(&mut self, rules: Rules, memos: &mut GameMemos) {
        self.round += 1;
        debug!("{}round {}", self.prefix, self.round);
        debug!("{}player 1 {:?}", self.prefix, self.players[0]);
        debug!("{}player 2 {:?}", self.prefix, self.players[1]);

        let repeats_previous_round = self.history.contains(&self.players[0]);
        self.history.insert(self.players[0].clone());

        let mut top_cards: Vec<(PlayerID, Card)> = 
            self.players.iter_mut().filter_map(|p| p.pop_front()).enumerate().collect();

        let mut winner = None;

        if rules == Rules::Recursive {
            if repeats_previous_round {
                winner = Some(0);

            } else if self.should_recurse(&top_cards) {
                match memos.get(&self.players[0]) {
                    Some(w) => {
                        winner = Some(*w);
                    }
                    None => {
                        let mut sub_game = self.make_sub_game(&top_cards);
                        sub_game.play_until_over(Rules::Recursive, memos);
                        let w = sub_game.winner();
                        winner = Some(w);
                        memos.insert(self.players[0].clone(), w);
                    }
                }
            }
        }

        if winner == None {
            // normal rules
            winner = top_cards.iter().max_by_key(|(_, card)| card).map(|(player, _)| *player);
        }

        let winner = winner.unwrap();

        top_cards.sort_by_key(|(player, _)| *player != winner);

        for (_, card) in top_cards {
            self.players[winner].push_back(card);
        }

    }

    fn over(&self) -> bool {
        self.players.iter().any(|p| p.is_empty())
    }

    fn play_until_over(&mut self, rules: Rules, memos: &mut GameMemos) {
        while !self.over() {
            self.play_round(rules, memos);
        }
    }

    fn winner(&self) -> PlayerID {
        self.players.iter().enumerate().find(|(_, cards)| !cards.is_empty()).unwrap().0
    }

    fn winning_score(&self) -> Score {
        let winner: &Player = self.players.iter().find(|p| !p.is_empty()).unwrap();
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
    let game = one_or_more(player).map(Game::new);
    game.parse(input)
}

// -- problems

fn part1(game: &mut Game) -> Score {
    game.play_until_over(Rules::Normal, &mut GameMemos::new()); 
    game.winning_score()
}

fn part2(game: &mut Game) -> Score {
    game.play_until_over(Rules::Recursive, &mut GameMemos::new()); 
    game.winning_score()
}

fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut game = parse_input(&input).unwrap().1;
    println!("part 1 {}", part1(&mut game));

    let mut game = parse_input(&input).unwrap().1;
    println!("part 2 {}", part2(&mut game))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_game() -> Game {
        Game::new(vec![
            vec![9, 2, 6, 3, 1].into_iter().collect(),
            vec![5, 8, 4, 7, 10].into_iter().collect()
        ])
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
        game.play_round(Rules::Normal, &mut GameMemos::new());
        assert_eq!(game.cards(0), vec![2, 6, 3, 1, 9, 5]);
        assert_eq!(game.cards(1), vec![8, 4, 7, 10]);

        game.play_round(Rules::Normal, &mut GameMemos::new());
        assert_eq!(game.cards(0), vec![6, 3, 1, 9, 5]);
        assert_eq!(game.cards(1), vec![4, 7, 10, 8, 2]);

        game.play_round(Rules::Normal, &mut GameMemos::new());
        assert_eq!(game.cards(0), vec![3, 1, 9, 5, 6, 4]);
        assert_eq!(game.cards(1), vec![7, 10, 8, 2]);
    }

    #[test]
    fn test_game_over() {
        let mut game = test_game();
        game.play_until_over(Rules::Normal, &mut GameMemos::new());
        assert_eq!(game.round, 29);
    }

    #[test]
    fn test_score() {
        let mut game = test_game();
        game.play_until_over(Rules::Normal, &mut GameMemos::new()); 
        assert_eq!(game.winning_score(), 306);
    }

    #[test]
    fn test_infinite_recursion_rule_works() {
        let mut game = Game::new(vec![
            vec![43, 19].into_iter().collect(),
            vec![2, 29, 14].into_iter().collect()
        ]);
        game.play_until_over(Rules::Recursive, &mut GameMemos::new());
    }

    #[test]
    fn test_play_recursive() {
        let mut game = test_game();
        game.play_until_over(Rules::Recursive, &mut GameMemos::new());
        
        assert_eq!(game.winning_score(), 291);
        assert_eq!(game.round, 17);
        assert_eq!(game.winner(), 1);
    }
}
