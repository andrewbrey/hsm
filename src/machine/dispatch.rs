use crate::util::log;
use crossterm::event::{poll, read, Event as CrossEvent, KeyCode, KeyModifiers};
use statig::blocking::InitializedStateMachine;
use std::{
    io::{self},
    time::Duration,
};

use super::{Event, MachineState};

pub fn generate_events(mut machine: InitializedStateMachine<MachineState>) -> io::Result<()> {
    loop {
        if poll(Duration::from_millis(1_000))? {
            let event = read()?;

            if let CrossEvent::Key(key) = event {
                match key.code {
                    KeyCode::Down => {
                        machine.handle(&Event::Press);
                    }
                    KeyCode::Left => {
                        machine.handle(&Event::TurnLeft);
                    }
                    KeyCode::Right => {
                        machine.handle(&Event::TurnRight);
                    }
                    KeyCode::Char('r') => {
                        if machine.removed {
                            machine.handle(&Event::Replace);
                        } else {
                            machine.handle(&Event::Remove);
                        }
                    }
                    KeyCode::Char('s') => {
                        log(format!(
                            "Current machine state is {:?} (removed: {}, previous_state: {:?})\n",
                            machine.state(),
                            machine.removed,
                            machine.previous_state
                        ));
                    }
                    KeyCode::Char('t') => {
                        machine.handle(&Event::IdleTimeout);
                    }
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
                    _ => {}
                };
            };
        }
    }

    Ok(())
}
