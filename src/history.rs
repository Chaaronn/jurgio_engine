use std::collections::HashMap;

const MAX_GAME_MOVES: usize = 1024;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct GameState {
    zobrist_hash: u64, // Unique identifier for board state
    half_move_clock: u16, // Moves since last pawn move or capture
}

impl GameState {
    pub fn new() -> Self {
        Self {
            zobrist_hash: 0,
            half_move_clock: 0,
        }
    }
}

pub struct History {
    list: [GameState; MAX_GAME_MOVES],
    count: usize,
    repetitions: HashMap<u64, usize>, // Tracks number of occurrences of a state
}

impl History {
    // Create a new history array containing game states.
    pub fn new() -> Self {
        Self {
            list: [GameState::new(); MAX_GAME_MOVES],
            count: 0,
            repetitions: HashMap::new(),
        }
    }

    // Put a new game state into the array.
    pub fn push(&mut self, g: GameState) {
        self.list[self.count] = g;
        self.count += 1;

        // Update repetition count for the state
        *self.repetitions.entry(g.zobrist_hash).or_insert(0) += 1;
    }

    // Return the last game state and decrement the counter.
    pub fn pop(&mut self) -> Option<GameState> {
        if self.count > 0 {
            self.count -= 1;
            let state = self.list[self.count];

            // Decrement repetition count
            if let Some(entry) = self.repetitions.get_mut(&state.zobrist_hash) {
                *entry -= 1;
                if *entry == 0 {
                    self.repetitions.remove(&state.zobrist_hash);
                }
            }

            Some(state)
        } else {
            None
        }
    }

    // Get a reference to a game state by index.
    pub fn get_ref(&self, index: usize) -> &GameState {
        &self.list[index]
    }

    // Get the number of states in the history.
    pub fn len(&self) -> usize {
        self.count
    }

    // Clear the history.
    pub fn clear(&mut self) {
        self.count = 0;
        self.repetitions.clear();
    }

    // Check if a state has repeated three or more times.
    pub fn is_threefold_repetition(&self) -> bool {
        self.repetitions.values().any(|&count| count >= 3)
    }

    // Check if the 50-move rule is applicable.
    pub fn is_fifty_move_rule(&self) -> bool {
        self.list[self.count - 1].half_move_clock >= 100
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_push_and_pop() {
        let mut history = History::new();
        let game_state1 = GameState {
            zobrist_hash: 12345,
            half_move_clock: 0,
        };
        let game_state2 = GameState {
            zobrist_hash: 67890,
            half_move_clock: 0,
        };

        history.push(game_state1);
        history.push(game_state2);

        assert_eq!(history.len(), 2);

        let popped = history.pop().unwrap();
        assert_eq!(popped, game_state2);
        assert_eq!(history.len(), 1);

        let popped = history.pop().unwrap();
        assert_eq!(popped, game_state1);
        assert_eq!(history.len(), 0);

        assert!(history.pop().is_none());
    }

    #[test]
    fn test_threefold_repetition() {
        let mut history = History::new();
        let game_state = GameState {
            zobrist_hash: 12345,
            half_move_clock: 0,
        };

        // Push the same state three times
        history.push(game_state);
        history.push(game_state);
        history.push(game_state);

        assert!(history.is_threefold_repetition());

        // Remove one occurrence
        history.pop();

        assert!(!history.is_threefold_repetition());
    }

    #[test]
    fn test_fifty_move_rule() {
        let mut history = History::new();
        let game_state = GameState {
            zobrist_hash: 12345,
            half_move_clock: 100, // 50 moves without pawn move or capture
        };

        history.push(game_state);

        assert!(history.is_fifty_move_rule());

        // Add a new state with less than 50 moves
        let game_state2 = GameState {
            zobrist_hash: 67890,
            half_move_clock: 90,
        };
        history.push(game_state2);

        assert!(!history.is_fifty_move_rule());
    }

    #[test]
    fn test_history_clear() {
        let mut history = History::new();
        let game_state = GameState {
            zobrist_hash: 12345,
            half_move_clock: 0,
        };

        history.push(game_state);
        history.push(game_state);
        history.clear();

        assert_eq!(history.len(), 0);
        assert!(!history.is_threefold_repetition());
    }

    #[test]
    fn test_get_ref() {
        let mut history = History::new();
        let game_state1 = GameState {
            zobrist_hash: 12345,
            half_move_clock: 0,
        };
        let game_state2 = GameState {
            zobrist_hash: 67890,
            half_move_clock: 0,
        };

        history.push(game_state1);
        history.push(game_state2);

        assert_eq!(history.get_ref(0), &game_state1);
        assert_eq!(history.get_ref(1), &game_state2);
    }
}
