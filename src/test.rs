use crate::test_result::TestResult;
use crate::test_runner::TestRunner;

pub trait Test {
    fn name(&self) -> &str;

    fn dependencies<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a>;

    fn run(&mut self, runner: &mut TestRunner) -> Box<dyn TestResult>;
}
