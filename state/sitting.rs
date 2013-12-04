// state/sitting.rs

use super::{State,Res};
use super::eating::Eating;

// Sitting is a bare type with no additional information.
pub struct Sitting;

impl State for Sitting {
	fn handle_input(&self, input: ~[~str]) -> Res {
		match input[0] {
			~"eat" => {
				if input.len() < 2 {
					Err(~"no food specified!")
				} else {
					self.eat(input[1])
				}
			}
			~"stare" => self.stare(),
			_        => Err(~"unknown command")
		}
	}
}

impl Sitting {
	fn eat(&self, food: ~str) -> Res {
		println("picked up a " + food);
		Ok(Some(~Eating{food: food} as ~State))
	}

	fn stare(&self) -> Res {
		println("the wall looks interesting");
		Ok(None)
	}
}
