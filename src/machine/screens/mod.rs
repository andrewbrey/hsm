use self::{
    menu::{handle_screen_menu, MenuState},
    ready::handle_screen_ready,
    temp_ctrl::{handle_screen_tc, TempCtrlState},
};
use super::{Event, ExternalCtx, MachineState};
use statig::{
    blocking,
    Response::{self, Handled, Super, Transition},
};

pub mod menu;
pub mod ready;
pub mod temp_ctrl;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Screen {
    Blank,
    Menu(MenuState),
    TempCtrl(TempCtrlState),
    KnobRemoved,
}

impl blocking::State<MachineState> for Screen {
    fn call_handler(
        &mut self,
        shared: &mut MachineState,
        event: &Event,
        ctx: &mut (),
    ) -> Response<Self> {
        match self {
            Screen::Blank => handle_screen_ready(event, shared, ctx),
            Screen::Menu(menu_state) => handle_screen_menu(menu_state, event, shared, ctx),
            Screen::TempCtrl(tc_state) => handle_screen_tc(tc_state, event, shared, ctx),
            Screen::KnobRemoved => Super,
        }
    }

    fn superstate(&mut self) -> Option<SuperScreen> {
        match self {
            Screen::Blank => Some(SuperScreen::ScreenOff),
            Screen::Menu(_) => Some(SuperScreen::ScreenOn),
            Screen::TempCtrl { .. } => Some(SuperScreen::ScreenOn),
            Screen::KnobRemoved => Some(SuperScreen::ScreenOn),
        }
    }
}

#[derive(Debug)]
pub enum SuperScreen {
    ScreenOff,
    ScreenOn,
}

impl blocking::Superstate<MachineState> for SuperScreen {
    fn call_handler(
        &mut self,
        shared: &mut MachineState,
        event: &Event,
        ctx: &mut (),
    ) -> Response<Screen> {
        match self {
            SuperScreen::ScreenOff => handle_screen_off(event, shared, ctx),
            SuperScreen::ScreenOn => handle_screen_on(event, shared, ctx),
        }
    }
}

fn handle_screen_off(
    event: &Event,
    shared: &mut MachineState,
    _ctx: &mut ExternalCtx,
) -> Response<Screen> {
    match event {
        Event::Press => {
            if !shared.removed {
                if matches!(shared.previous_state, Screen::Blank | Screen::KnobRemoved) {
                    return Transition(shared.previous_state);
                } else {
                    return Transition(Screen::Menu(MenuState::default()));
                }
            }

            Handled
        }
        Event::Remove => {
            shared.removed = true;

            Handled
        }
        Event::Replace => {
            shared.removed = false;

            Handled
        }
        _ => Super,
    }
}

fn handle_screen_on(
    event: &Event,
    shared: &mut MachineState,
    _ctx: &mut ExternalCtx,
) -> Response<Screen> {
    match event {
        Event::IdleTimeout => Transition(Screen::Blank),
        Event::Remove => {
            shared.removed = true;

            Transition(Screen::KnobRemoved)
        }
        Event::Replace => {
            shared.removed = false;

            Transition(shared.previous_state)
        }
        _ => Super,
    }
}
