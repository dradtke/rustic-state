// state/eating.rs

use super::{State,Res,invalid_input};
use super::sitting::Sitting;

// Eating is a state that needs to keep track of which food
// it was created with.
pub struct Eating {
	food: ~str
}

impl State for Eating {
	fn handle_input(&self, input: ~[~str]) -> Res {
		match input[0] {
			~"chew" => self.chew(),
			~"stop" => self.stop(),
			_       => invalid_input()
		}
	}
}

impl Eating {
	fn chew(&self) -> Res {
		println("took a bite out of " + self.food);
		Ok(None)
	}

	fn stop(&self) -> Res {
		println("put down " + self.food);
		Ok(Some(~Sitting as ~State))
	}
}
