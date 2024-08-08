use serde::{Deserialize, Serialize};

pub trait Output {
    fn get_n(&self) -> usize;
    fn update_state(&mut self, new_state: Vec<bool>);
}

#[derive(Deserialize, Serialize)]
pub enum OutputType {
    Gate,
}

pub struct Door {
    pub state: bool,
}
impl Output for Door {
    fn get_n(&self) -> usize {
        return 1;
    }
    fn update_state(&mut self, new_state: Vec<bool>) {
        self.state = new_state[0];
    }
}
