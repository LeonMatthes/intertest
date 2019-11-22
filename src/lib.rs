pub mod test;
pub mod test_case;
pub mod test_suite;

pub static mut INTERTEST_GLOBAL_TEST_SUITES: Option<test_suite::TestSuite> = None;

#[macro_export]
macro_rules! intertest_main {
    ($($suite:expr),*) => {
        fn main() {
            let mut global_suite = TestSuite::new(String::from(""));
            $( global_suite.add_test(Box::new($suite())); )*
            global_suite.run();
        }
    };
}
