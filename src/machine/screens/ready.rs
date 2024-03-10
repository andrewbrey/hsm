use super::{menu::MenuState, Screen};
use crate::machine::{Event, ExternalCtx, MachineState};
use statig::Response::{self, Super, Transition};

pub fn handle_screen_ready(
    event: &Event,
    _shared: &mut MachineState,
    _ctx: &mut ExternalCtx,
) -> Response<Screen> {
    match event {
        Event::Press => Transition(Screen::Menu(MenuState::default())),
        _ => Super,
    }
}
