pub mod macros;
pub mod test;
pub mod test_case;
pub mod test_runner;
pub mod test_suite;

pub static mut INTERTEST_GLOBAL_TEST_SUITES: Option<test_suite::TestSuite> = None;
