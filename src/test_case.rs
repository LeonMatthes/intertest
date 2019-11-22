use crate::test::{Test, TestResult, TestResult::*};
use std::{thread, vec::Vec};

pub struct TestCase<F>
where
    F: Fn() + Send + Sync + 'static,
{
    name: String,
    dependencies: Vec<String>,
    result: TestResult,
    test_function: &'static F,
}

impl<F> TestCase<F>
where
    F: Fn() + Send + Sync + 'static,
{
    pub fn new(name: String, test_function: &'static F) -> TestCase<F> {
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
        test_function: &'static F,
    ) -> TestCase<F> {
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

impl<F> Test for TestCase<F>
where
    F: Fn() + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn dependencies<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(self.dependencies.iter())
    }

    fn result(&self) -> &TestResult {
        &self.result
    }

    fn run(&mut self) -> &TestResult {
        let join_handle = thread::spawn(self.test_function);

        self.result = match join_handle.join() {
            Ok(_) => Success,
            Err(_) => Error,
        };
        &self.result
    }
}
