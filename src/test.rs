use crate::test_runner::TestRunner;

#[derive(Clone, PartialEq, Debug)]
pub enum TestResult {
    Error,
    Ignored,
    Success,
    NotRun,
}

pub trait Test {
    fn name(&self) -> &str;

    fn dependencies<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a>;

    fn result(&self) -> &TestResult;

    fn run(&mut self, runner: &mut TestRunner) -> &TestResult;
}
