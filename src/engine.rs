use crate::io::IO;

pub const REPL_MAIN: &str = "repl_main";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainFunction {
    Main,
    ReplMain,
    SmainStdin,
    SmainStdinLines,
}

impl MainFunction {
    pub fn name(&self) -> &str {
        match self {
            MainFunction::Main => "main",
            MainFunction::ReplMain => REPL_MAIN,
            _ => "smain",
        }
    }
}

pub trait Engine<I: IO>: Clone {
    fn new(fs: I) -> Self;

    fn run_main(&self, module: &str, main: MainFunction, show_output: bool);

    fn has_var(&self, index: usize) -> bool;

    fn run_tests(&self, modules: &[&str]);

    fn interrupt(&self);
}
