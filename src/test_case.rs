use crate::test::Test;
use crate::test_result::TestResult;
use crate::test_runner::TestRunner;
use std::{panic::RefUnwindSafe, vec::Vec};

pub struct TestCase {
    name: String,
    dependencies: Vec<String>,
    pub test_function: &'static (dyn Fn() + RefUnwindSafe),
}

impl TestCase {
    pub fn new(name: String, test_function: &'static (dyn Fn() + RefUnwindSafe)) -> TestCase {
        TestCase {
            name,
            dependencies: Vec::new(),
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

    fn run(&mut self, runner: &mut TestRunner) -> Box<dyn TestResult> {
        Box::from(runner.run_case(self))
    }
}
