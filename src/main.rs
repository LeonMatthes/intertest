use intertest::macros::*;
use intertest::{child_suite, intertest_main, test_case, test_suite};

test_suite! { do_nothing:
    test_case! { do_nothing_too:
        println!("nothing");
        // panic!("AAAAH");
    }
}

test_suite! { hello[do_nothing]:
    test_case! { error :
        println!("Let me error this real quick!");
        // panic!("ERROR!");
    }
    test_case! { world[error]:
        println!("Hello, world");
    }

    child_suite! { moar_tests[error]:
        test_case! { my_test:
            println!("my_test");
        }

        test_case! { another_test:
            println!("another_test");
        }
    }
}

intertest_main!(do_nothing, hello);
