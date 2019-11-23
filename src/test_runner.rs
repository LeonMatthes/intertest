use crate::{
    test::{Test, TestResult},
    test_case::TestCase,
    test_suite::TestSuite,
};
use graphlib::VertexId;
use std::thread;

pub struct TestRunner {
    results: Vec<TestResult>,
    recursion: i64,
}

impl TestRunner {
    pub fn new() -> TestRunner {
        TestRunner {
            results: vec![],
            recursion: 0,
        }
    }

    fn print_result(&self, test: &Box<dyn Test>) {
        for _ in 0..self.recursion {
            print!("  ")
        }
        match *test.result() {
            TestResult::Error => print!("E"),
            TestResult::Ignored => print!("*"),
            TestResult::Success | TestResult::NotRun => print!("."),
        }

        println!(" {}", test.name());
    }

    pub fn run_suite(&mut self, suite: &mut TestSuite) -> TestResult {
        self.recursion += 1;
        let mut success = true;
        if !suite.tests.is_cyclic() {
            let topo = suite
                .tests
                .topo()
                .map(VertexId::clone)
                .collect::<Vec<VertexId>>();

            for test_index in &topo {
                if suite.check_dependencies(test_index) {
                    if let Some(test) = suite.tests.fetch_mut(test_index) {
                        let result = test.run(self);
                        if *result != TestResult::Success {
                            self.results.push(result.clone());
                            success = false;
                        }
                        self.print_result(test);
                    }
                }
            }
        }
        self.recursion -= 1;
        if success {
            TestResult::Success
        } else {
            TestResult::Error
        }
    }

    pub fn run_case(&mut self, case: &TestCase) -> TestResult {
        let join_handle = thread::spawn(case.test_function);

        match join_handle.join() {
            Ok(_) => TestResult::Success,
            Err(_) => TestResult::Error,
        }
    }
}
