use super::{menu::MenuState, Screen};
use crate::machine::{Event, ExternalCtx, MachineState};
use statig::Response::{self, Handled, Super, Transition};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct TempCtrlState {
    pub target: f32,
}

impl Default for TempCtrlState {
    fn default() -> Self {
        Self { target: 20.0 }
    }
}

pub fn handle_screen_tc(
    tc_state: &mut TempCtrlState,
    event: &Event,
    _shared: &mut MachineState,
    _ctx: &mut ExternalCtx,
) -> Response<Screen> {
    match event {
        Event::TurnRight => {
            tc_state.target += 1.0;

            Handled
        }
        Event::TurnLeft => {
            if tc_state.target <= 20.0 {
                return Transition(Screen::Menu(MenuState::default()));
            }

            tc_state.target -= 1.0;

            Handled
        }
        Event::IdleTimeout => Handled,
        _ => Super,
    }
}
