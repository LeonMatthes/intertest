use intertest::{intertest_main, test_case, test_suite};
use intertest::{test::Test, test_suite::TestSuite};

test_suite!(do_nothing {
    test_case!(do_nothing_too {
        println!("nothing");
        panic!("AAAAH");
    })
});

test_suite!(hello[do_nothing] { 
    test_case!(error {
        println!("Let me error this real quick!");
        panic!("ERROR!");
    });
    test_case!(world[error] {
        println!("Hello, world");
    });
});

intertest_main!(do_nothing, hello);
