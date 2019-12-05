use crate::{test::Test, test_case::TestCase, test_suite::TestSuite};
use backtrace::Backtrace;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct ErrorInfo {
    pub back_trace: Backtrace,
    pub message: String,
}

#[derive(Clone, Debug)]
pub enum TestRunResult {
    Error(Option<ErrorInfo>),
    Success,
    Ignored,
}

use TestRunResult::*;

impl std::cmp::PartialEq for TestRunResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error(_), Error(_)) => true,
            (Ignored, Ignored) => true,
            (Success, Success) => true,
            // (NotRun, NotRun) => true,
            _ => false,
        }
    }
}

impl std::cmp::Eq for TestRunResult {
    // no implementation needed, provided by PartialEq
}

impl Hash for TestRunResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Error(_) => 0,
            Ignored => 1,
            Success => 2,
            // NotRun => 3,
        }
        .hash(state)
    }
}

pub trait TestResult {
    fn print(&self, parent_path: &str);

    fn run_result(&self) -> TestRunResult;
}

pub struct TestCaseResult {
    name: String,
    run_result: TestRunResult,
}

impl TestCaseResult {
    pub fn new(case: &TestCase, run_result: TestRunResult) -> TestCaseResult {
        TestCaseResult {
            name: case.name().to_string(),
            run_result,
        }
    }
}

impl TestResult for TestCaseResult {
    fn print(&self, parent_path: &str) {
        if let Error(err) = &self.run_result {
            println!("Test Failure: {}/{}", parent_path, &self.name);
            match err {
                Some(info) => {
                    println!("Message: {}", info.message);
                    println!("{:?}", info.back_trace);
                }
                None => println!("No backtrace available!"),
            }
            println!("");
        }
    }

    fn run_result(&self) -> TestRunResult {
        self.run_result.clone()
    }
}

pub struct TestSuiteResult {
    name: String,
    child_results: Vec<Box<dyn TestResult>>,
}

impl TestSuiteResult {
    pub fn new(suite: &TestSuite, child_results: Vec<Box<dyn TestResult>>) -> TestSuiteResult {
        TestSuiteResult {
            name: suite.name().to_string(),
            child_results,
        }
    }
}

impl TestResult for TestSuiteResult {
    fn print(&self, parent_path: &str) {
        if self.run_result() != Success {
            let my_path = format!("{}/{}", parent_path, &self.name);
            self.child_results
                .iter()
                .for_each(|result| result.print(&my_path));
        }
    }

    fn run_result(&self) -> TestRunResult {
        if self
            .child_results
            .iter()
            .all(|result| result.run_result() == Success)
        {
            Success
        } else {
            Error(None)
        }
    }
}
