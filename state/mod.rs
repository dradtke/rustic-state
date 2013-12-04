// state/mod.rs

mod sitting;
mod eating;

// Res represents the result of a state trying to handle input. It
// can be one of three things: an error (such as invalid input),
// a new state (successful input that changes the state), or neither
// (successful input that does not change the state). It will be
// up to the Hero to use that result appropriately.
pub type Res = Result<Option<~State>, ~str>;

// State represents a state for our Hero.
pub trait State : Send {
	fn handle_input(&self, ~[~str]) -> Res;
}

