use intertest::macros::*;
use intertest::{child_suite, intertest_main, test_case, test_suite};

test_suite! { do_nothing:
    test_case! { do_nothing_too:
        // panic!("AAAAH");
    }
}

test_suite! { hello[do_nothing]:

    test_case! { error :
        panic!("ERROR!");
    }

    test_case! { world[error]:
    }

    child_suite! { moar_tests[world]:
        test_case! { my_test:
        }

        test_case! { another_test:
        }
    }
}

intertest_main!(do_nothing, hello);
