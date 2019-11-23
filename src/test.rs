use crate::test_runner::TestRunner;
use backtrace::Backtrace;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct ErrorInfo {
    pub back_trace: Backtrace,
    pub message: String,
}

#[derive(Clone, Debug)]
pub enum TestResult {
    Error(Option<ErrorInfo>),
    Ignored,
    Success,
    NotRun,
}

use TestResult::*;

impl std::cmp::PartialEq for TestResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error(_), Error(_)) => true,
            (Ignored, Ignored) => true,
            (Success, Success) => true,
            (NotRun, NotRun) => true,
            _ => false,
        }
    }
}

impl std::cmp::Eq for TestResult {}

impl Hash for TestResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Error(_) => 0,
            Ignored => 1,
            Success => 2,
            NotRun => 3,
        }
        .hash(state)
    }
}

pub trait Test {
    fn name(&self) -> &str;

    fn dependencies<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a>;

    fn result(&self) -> &TestResult;

    fn run(&mut self, runner: &mut TestRunner) -> &TestResult;

    fn ignore(&mut self);
}
