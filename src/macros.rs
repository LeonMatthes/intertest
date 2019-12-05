pub use crate::{test::Test, test_runner::TestRunner, test_suite::TestSuite};

#[macro_export]
macro_rules! intertest_main {
    ($($suite:expr),*) => {
        fn main() {
            let mut global_suite = TestSuite::new(String::from(""));
            $( global_suite.add_test(Box::new($suite())); )*

            intertest::intertest_main_function(global_suite)
        }
    };
}

#[macro_export]
macro_rules! child_suite {
    { $name:ident[$($dependency:ident),*$(,)?]: $($test:expr)* } => {
        {
        let mut suite = $crate::test_suite::TestSuite::new_with_dependencies(
            String::from(stringify!($name)),
            vec![$(String::from(stringify!($dependency)),)*]);
        $( suite.add_test(Box::new($test)); )*
        suite
        }
    };
    { $name:ident: $($test:expr)* } => {
        child_suite! {$name[]:
            $($test)*
        }
    }
}

#[macro_export]
macro_rules! test_suite {
    { $name:ident[$($dependency:ident),*]: $($test:expr)* } => {
        fn $name() -> TestSuite {
            child_suite! { $name[$($dependency,)*]:
                $($test)*
            }
        }
    };
    { $name:ident : $($test:expr)* } => {
        fn $name() -> TestSuite {
            child_suite! { $name[]:
                $($test)*
            }
        }
    }
}

#[macro_export]
macro_rules! test_case {
    {$name:ident[$($dependency:ident),*]: $($test:stmt);*$(;)?} => {{
        fn case_function() {
            $($test;)*
        }
        $crate::test_case::TestCase::new_with_dependencies(
            String::from(stringify!($name)),
            vec![$( String::from(stringify!($dependency)) )* ],
            &case_function)
    }};
    ($name:ident: $($test:stmt);*$(;)?) => {
        test_case! { $name[]:
            $($test;)*
        }
    };
}
