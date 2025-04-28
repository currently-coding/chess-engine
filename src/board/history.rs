use super::GameState;
use crate::defs::*;

#[derive(Clone, Copy)]
pub struct GameHistory {
    list: [GameState; MAX_GAME_MOVES],
    count: usize,
}
impl GameHistory {
    pub fn new() -> Self {
        Self {
            list: [GameState::new(); MAX_GAME_MOVES],
            count: 0,
        }
    }

    // just changing count var - not deleting values
    pub fn push(&mut self, new_state: GameState) {
        self.list[self.count + 1] = new_state;
        self.count += 1;
    }
    pub fn pop(&mut self) -> Option<GameState> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.list[self.count])
        } else {
            None
        }
    }
    pub fn get_current(&self) -> GameState {
        self.list[self.count]
    }
    pub fn unmake(&mut self) -> GameState {
        self.count -= 1;
        self.get_current()
    }
    pub fn get_ref(&self, num: usize) -> Option<&GameState> {
        if num <= self.count {
            Some(&self.list[num])
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn clear(&mut self) {
        self.count = 0;
    }
    pub fn previous(&self) -> Option<&GameState> {
        self.get_ref(self.count - 1)
    }
}
