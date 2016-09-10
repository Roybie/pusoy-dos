use game::player_move::Move;

/// single round
#[derive(Clone, Debug, PartialEq)]
pub struct Round{
    players: Vec<i32>,
    current_player: i32,
    last_move: Move
}

impl Round {

    /// create new round
    pub fn new(players: Vec<i32>, current_player: i32, last_move: Move) -> Round {

        if !players.contains(&current_player){
            panic!("current player needs to be in the pool of players");
        }

        Round{
            players: players,
            current_player: current_player,
            last_move: last_move
        }
    }

    /// play a move in the current round
    pub fn play(&self, player_id: i32, new_move: Move) -> Result<Round, Round> {

        if player_id != self.current_player {
            return Err(self.clone());
        }

        let next_player = if self.current_player == *self.players.last().unwrap() {
            self.players.first().unwrap()
        } else {
            let index = self.players.binary_search(&self.current_player).unwrap();
            self.players.get(index + 1).unwrap()
        };
        
        if self.last_move == Move::Pass || new_move == Move::Pass {
        
            Ok(Round{
                players: self.players.clone(), 
                current_player: *next_player,
                last_move: self.last_move
            })
        } else if self.valid_move(new_move) {

            Ok(Round{
                players: self.players.clone(), 
                current_player: *next_player,
                last_move: new_move
            })
        } else {
            Err(self.clone())
        }
    }
    
    /// check who should play the next move
    pub fn get_next_player(&self) -> i32 {
        self.current_player
    }

    fn valid_move(&self, new_move: Move) -> bool {
        let matching_type = match self.last_move {
            Move::Single(_) => {
                match new_move {
                    Move::Single(_) => true,
                    _               => false
                }
            },
            Move::Pair(_, _) => {
                match new_move {
                    Move::Pair(_,_) => true,
                    _               => false
                }
            },
            Move::Prial(_, _, _) => {
                match new_move {
                    Move::Prial(_,_,_) => true,
                    _               => false
                }
            },
            Move::FiveCardTrick(_,_,_,_,_) => {
                match new_move {
                    Move::FiveCardTrick(_,_,_,_,_) => true,
                    _               => false
                }
            },
            _ => false
        };

        matching_type && new_move > self.last_move
    }
}
