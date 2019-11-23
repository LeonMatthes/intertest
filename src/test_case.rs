use crate::test::{Test, TestResult, TestResult::*};
use crate::test_runner::TestRunner;
use std::{panic::RefUnwindSafe, vec::Vec};

pub struct TestCase {
    name: String,
    dependencies: Vec<String>,
    result: TestResult,
    pub test_function: &'static (dyn Fn() + RefUnwindSafe),
}

impl TestCase {
    pub fn new(name: String, test_function: &'static (dyn Fn() + RefUnwindSafe)) -> TestCase {
        TestCase {
            name,
            dependencies: Vec::new(),
            result: NotRun,
            test_function,
        }
    }

    pub fn new_with_dependencies(
        name: String,
        dependencies: Vec<String>,
        test_function: &'static (dyn Fn() + RefUnwindSafe),
    ) -> TestCase {
        TestCase {
            name,
            result: NotRun,
            dependencies,
            test_function,
        }
    }

    pub fn add_dependencies(&mut self, dependencies: &[String]) {
        self.dependencies.extend_from_slice(dependencies);
    }
}

impl Test for TestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn dependencies<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(self.dependencies.iter())
    }

    fn result(&self) -> &TestResult {
        &self.result
    }

    fn run(&mut self, runner: &mut TestRunner) -> &TestResult {
        self.result = runner.run_case(self);
        &self.result
    }

    fn ignore(&mut self) {
        self.result = TestResult::Ignored;
    }
}
