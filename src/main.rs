use crate::machine::run_machine;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use std::io::{self};

mod machine;
mod util;

const HELP: &str = r#"Tinker with HSM
 ↓  press
 ←  turn left
 →  turn right
 r  remove / replace (toggle)
 t  idle timeout
 s  print machine state

 Esc / q / Ctrl+c to quit
"#;

fn main() -> io::Result<()> {
    println!("{}", HELP);

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        execute!(std::io::stderr(), LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout)?;

    run_machine();

    execute!(stdout)?;
    disable_raw_mode()
}
