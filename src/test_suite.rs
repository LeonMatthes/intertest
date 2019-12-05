use crate::test::Test;
use crate::test_result::TestResult;
use crate::test_runner::TestRunner;
use std::vec::Vec;

pub struct TestSuite {
    pub tests: Vec<Box<dyn Test>>,
    name: String,
    dependencies: Vec<String>,
}

impl TestSuite {
    pub fn new(name: String) -> TestSuite {
        TestSuite::new_with_dependencies(name, Vec::new())
    }

    pub fn new_with_dependencies(name: String, dependencies: Vec<String>) -> TestSuite {
        TestSuite {
            tests: Vec::new(),
            name,
            dependencies,
        }
    }

    fn assert_dependencies_exist(&self, test: &dyn Test) {
        for dependency in test.dependencies() {
            if !self.tests.iter().any(|test| test.name() == dependency) {
                panic!(
                    "Dependency '{}' of test '{}' could not be found in suite '{}'",
                    dependency,
                    test.name(),
                    self.name
                );
            }
        }
    }

    pub fn add_test(&mut self, test: Box<dyn Test>) {
        self.assert_dependencies_exist(&*test);

        self.tests.push(test);
    }
}

impl Test for TestSuite {
    fn name(&self) -> &str {
        &self.name
    }

    fn dependencies<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(self.dependencies.iter())
    }

    fn run(&mut self, runner: &mut TestRunner) -> Box<dyn TestResult> {
        Box::from(runner.run_suite(self))
    }
}
