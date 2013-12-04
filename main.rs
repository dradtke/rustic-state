// main.rs

mod state;

struct Hero { st: ~state::State }
impl Hero {
	// Create a new Hero.
	fn new() -> Hero {
		Hero {
			st: ~state::sitting::Sitting as ~state::State
		}
	}

	// Handle input. The input is checked to make sure it's non-empty,
	// then passed to the Hero's state. If the state returns a new state
	// value, then update the Hero instance.
	fn handle_input(&mut self, input: ~[~str]) {
		if input.len() == 0 {
			return;
		}
		match self.st.handle_input(input) {
			Ok(new_state) => match new_state {
				Some(st) => self.st = st,
				None => ()
			},
			Err(msg) => println(msg)
		}
	}
}

fn main() {
	let mut hero = ~Hero::new();
	hero.handle_input(~[~"eat", ~"burger"]);
	hero.handle_input(~[~"chew"]);
	hero.handle_input(~[~"chew"]);
	hero.handle_input(~[~"stop"]);
	hero.handle_input(~[~"stare"]);
}
