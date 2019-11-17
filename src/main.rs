fn aaaaah_panic() {
    println!("aaaaah_panic");
    panic!("test");
}

fn another_depency() {
    println!("another dependency");
}

fn hello_world() {
    println!("Hello, World!");
    assert_eq!(true, true); // this is true
}

use intertest::{test::Test, test_case::TestCase, test_suite::TestSuite};

fn main() {
    let mut suite = TestSuite::new(String::from("My Suite"));

    let dependency1 = TestCase::new_with_dependencies(
        String::from("panic!"),
        vec![String::from("another dependency")],
        &aaaaah_panic,
    );
    let dependency2 = TestCase::new(String::from("another dependency"), &another_depency);
    let case = TestCase::new_with_dependencies(
        String::from("so needy"),
        vec![String::from("panic!"), String::from("another dependency")],
        &hello_world,
    );

    suite.add_test(Box::new(dependency2));
    suite.add_test(Box::new(dependency1));
    suite.add_test(Box::new(case));

    println!("Result: {:?}", suite.run());
}
