use crate::{
    test::{ErrorInfo, Test, TestResult},
    test_case::TestCase,
    test_suite::TestSuite,
};
use backtrace::Backtrace;
use graphlib::VertexId;
use std::{collections::HashMap, io::Write, panic, sync::Arc, sync::Mutex};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct TestRunner {
    errors: Vec<TestResult>,
    recursion: i64,
    out_stream: StandardStream,
    colors: HashMap<TestResult, ColorSpec>,
}

impl TestRunner {
    fn insert_colors(&mut self) {
        let mut success_color = ColorSpec::new();
        success_color.set_fg(Some(Color::Green));
        self.colors.insert(TestResult::Success, success_color);

        let mut error_color = ColorSpec::new();
        error_color.set_fg(Some(Color::Red));
        self.colors.insert(TestResult::Error(None), error_color);

        let mut ignore_color = ColorSpec::new();
        ignore_color.set_fg(Some(Color::Rgb(150, 150, 150)));
        self.colors.insert(TestResult::Ignored, ignore_color);
    }

    pub fn new() -> TestRunner {
        let mut runner = TestRunner {
            errors: Vec::new(),
            recursion: 0,
            out_stream: StandardStream::stdout(ColorChoice::Auto),
            colors: HashMap::new(),
        };
        runner.insert_colors();
        runner
    }

    fn print_result(&mut self, test: &Box<dyn Test>) {
        let mut padding = String::new();
        for _ in 0..self.recursion {
            padding.push_str("  ");
        }

        if let Some(color) = self.colors.get(test.result()) {
            self.out_stream.set_color(color).ok();
        }

        let result_string = match *test.result() {
            TestResult::Error(_) => "E",
            TestResult::Ignored => ".",
            TestResult::Success | TestResult::NotRun => "*",
        };

        if let Err(_) = writeln!(
            &mut self.out_stream,
            "{}{} {}",
            padding,
            result_string,
            test.name()
        ) {
            println!("Could not write result to output stream");
        }

        self.out_stream.reset().ok();
        // flush to make sure the reset completed and we don't continue painting further output
        self.out_stream.flush().ok();
    }

    pub fn print_errors(&mut self) {
        for result in &self.errors {
            if let TestResult::Error(Some(error)) = result {
                writeln!(&mut self.out_stream, "{:?}", error.back_trace).ok();
            }
        }
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
                        if *result == TestResult::Error(None) {
                            self.errors.push(result.clone());
                            success = false;
                        }
                        self.print_result(test);
                    }
                } else {
                    if let Some(test) = suite.tests.fetch_mut(test_index) {
                        test.ignore();
                        self.print_result(test);
                    }
                }
            }
        }
        self.recursion -= 1;
        if success {
            TestResult::Success
        } else {
            TestResult::Error(None)
        }
    }

    pub fn run_case(&mut self, case: &TestCase) -> TestResult {
        let old_hook = panic::take_hook();

        let error_info: Arc<Mutex<Option<ErrorInfo>>> = Arc::new(Mutex::new(None));
        {
            let error_info = error_info.clone();
            panic::set_hook(Box::new(move |_| {
                *error_info.lock().unwrap() = Some(ErrorInfo {
                    back_trace: Backtrace::new(),
                    message: String::from(""),
                });
            }));
        }

        // Error will be collected by panic hook
        panic::catch_unwind(case.test_function).ok();

        panic::set_hook(old_hook);
        let info = error_info.lock().unwrap().clone();
        match info {
            Some(info) => TestResult::Error(Some(info.clone())),
            None => TestResult::Success,
        }
    }
}
