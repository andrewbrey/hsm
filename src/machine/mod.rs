use self::{
    dispatch::generate_events,
    screens::{Screen, SuperScreen},
};
use crate::util::log;
use statig::blocking::IntoStateMachineExt;
use statig::{prelude::StateOrSuperstate, IntoStateMachine};

mod dispatch;
mod screens;

pub fn run_machine() {
    let machine = MachineState {
        removed: false,
        previous_state: MachineState::INITIAL,
    }
    .uninitialized_state_machine()
    .init();

    log(format!(
        "> Machine started, current state is {:?}\n",
        machine.state()
    ));

    generate_events(machine).unwrap();
}

#[derive(Debug)]
pub enum Event {
    Press,
    TurnRight,
    TurnLeft,
    Remove,
    Replace,
    IdleTimeout,
}

#[derive(Debug, Clone, Copy)]
pub struct MachineState {
    pub removed: bool,
    pub previous_state: Screen,
}

pub type ExternalCtx = ();

impl IntoStateMachine for MachineState {
    type State = Screen;

    type Superstate<'sup> = SuperScreen;

    type Event<'evt> = Event;

    type Context<'ctx> = ExternalCtx;

    const INITIAL: Screen = Screen::Blank;

    const ON_TRANSITION: fn(&mut Self, &Self::State, &Self::State) = |shared, source, target| {
        if !matches!(source, Screen::Blank | Screen::KnobRemoved) {
            shared.previous_state = *source;
        }

        let msg = format!(
            "\n> Transitioned from {source:?} to {target:?} with previous set to {:?}\n",
            shared.previous_state
        );

        log(msg);
    };

    const ON_DISPATCH: fn(&mut Self, StateOrSuperstate<'_, '_, Self>, &Self::Event<'_>) =
        |_shared, state, event| {
            log(format!("   dispatching {event:?} to {state:?}\n"));
        };
}
