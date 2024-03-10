use super::{temp_ctrl::TempCtrlState, Screen};
use crate::machine::Event;
use crate::machine::{ExternalCtx, MachineState};
use statig::Response::{self, Handled, Super, Transition};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum MenuItem {
    TempControl,
    Off,
    PowerControl,
    KnobVis,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct MenuState {
    pub hovered: MenuItem,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            hovered: MenuItem::Off,
        }
    }
}

impl MenuState {
    pub fn next_hover(&mut self, event: &Event) {
        let next_hover: MenuItem = match event {
            Event::TurnRight => match self.hovered {
                MenuItem::TempControl => MenuItem::Off,
                MenuItem::Off => MenuItem::PowerControl,
                MenuItem::PowerControl => MenuItem::KnobVis,
                _ => self.hovered,
            },
            Event::TurnLeft => match self.hovered {
                MenuItem::Off => MenuItem::TempControl,
                MenuItem::PowerControl => MenuItem::Off,
                MenuItem::KnobVis => MenuItem::PowerControl,
                _ => self.hovered,
            },
            _ => self.hovered,
        };

        self.hovered = next_hover;
    }
}

pub fn handle_screen_menu(
    menu_state: &mut MenuState,
    event: &Event,
    _shared: &mut MachineState,
    _ctx: &mut ExternalCtx,
) -> Response<Screen> {
    match event {
        Event::Press => match menu_state.hovered {
            MenuItem::TempControl => Transition(Screen::TempCtrl(TempCtrlState::default())),
            MenuItem::Off => Transition(Screen::Blank),
            _ => Handled,
        },
        Event::TurnLeft | Event::TurnRight => {
            menu_state.next_hover(event);

            if matches!(menu_state.hovered, MenuItem::TempControl) {
                return Transition(Screen::TempCtrl(TempCtrlState::default()));
            }

            Handled
        }
        _ => Super,
    }
}
