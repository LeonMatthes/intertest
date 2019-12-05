pub mod macros;
pub mod test;
pub mod test_case;
pub mod test_result;
pub mod test_runner;
pub mod test_suite;

pub static mut INTERTEST_GLOBAL_TEST_SUITES: Option<test_suite::TestSuite> = None;

use test::Test;
use test_runner::TestRunner;
use test_suite::TestSuite;

pub fn intertest_main_function(mut suite: TestSuite) {
    let mut runner = TestRunner::new();
    let result = suite.run(&mut runner);
    result.print("");
}
