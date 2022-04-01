use crate::parsers::in_release::in_release_builder::InReleaseBuilder;
use crate::parsers::in_release::in_release_states::{ParserState, State};

pub struct NoOpState;

impl NoOpState {
    pub fn new() -> Self {
        return NoOpState;
    }
}

impl ParserState for NoOpState {
    fn parse_line(&mut self, line: &str) -> Result<(), ()> {
        return Result::Ok(());
    }

    fn is_my_state(&self, line: &str) -> bool {
        return false;
    }

    fn get_state(&self) -> State {
        return State::NoOp;
    }

    fn dump_data(&mut self, target: &mut InReleaseBuilder) {
        return;
    }
}
