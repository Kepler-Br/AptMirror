use crate::parsers::in_release::in_release_builder::InReleaseBuilder;

pub enum State {
    NoOp,
    Header,
    Files,
    Signature,
}

pub enum InReleaseParseError {
    Unexpected
}

pub trait ParserState {
    fn parse_line(&mut self, line: &str) -> Result<(), ()>;

    fn is_my_state(&self, line: &str) -> bool;

    fn get_state(&self) -> State;

    fn dump_data(&mut self, target: &mut InReleaseBuilder);
}
