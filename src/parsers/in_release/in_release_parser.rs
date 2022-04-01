use std::collections::VecDeque;
use crate::parsers::in_release::in_release_builder::InReleaseBuilder;
use crate::parsers::in_release::in_release_states::ParserState;
use crate::parsers::in_release::states::{FilesState, HeaderState, NoOpState};
use crate::parsers::InRelease;

type BoxedState = Box<dyn ParserState>;

pub fn parse_in_release(content: &str) -> Result<InRelease, ()> {
    let v: Vec<BoxedState> = vec![
        Box::new(NoOpState::new()),
        // Box::new(HeaderState::new()),
        Box::new(FilesState::new()),
    ];
    let mut states: VecDeque<BoxedState> = VecDeque::from(v);

    let mut current_state = states.pop_back()
        .expect("State vector is empty");

    let mut result = InReleaseBuilder::new();
    for line in content.lines() {
        if !states.is_empty() {
            current_state.dump_data(&mut result);

            let next_state = states.back().unwrap();

            if next_state.is_my_state(line) {
                current_state = states.pop_back().unwrap();
            }
        }

        current_state.parse_line(line)?;
    }

    return Result::Err(());
}
