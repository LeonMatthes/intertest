use crate::{
    test::Test,
    test_case::TestCase,
    test_result::{ErrorInfo, TestCaseResult, TestResult, TestRunResult, TestSuiteResult},
    test_suite::TestSuite,
};
use backtrace::Backtrace;
use std::{collections::HashMap, io::Write, panic, sync::Arc, sync::Mutex};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct TestRunner {
    recursion: i64,
    out_stream: StandardStream,
    colors: HashMap<TestRunResult, ColorSpec>,
}

impl TestRunner {
    fn insert_colors(&mut self) {
        let mut success_color = ColorSpec::new();
        success_color.set_fg(Some(Color::Green));
        self.colors.insert(TestRunResult::Success, success_color);

        let mut error_color = ColorSpec::new();
        error_color.set_fg(Some(Color::Red));
        self.colors.insert(TestRunResult::Error(None), error_color);

        let mut ignore_color = ColorSpec::new();
        ignore_color.set_fg(Some(Color::Rgb(150, 150, 150)));
        self.colors.insert(TestRunResult::Ignored, ignore_color);
    }

    pub fn new() -> TestRunner {
        let mut runner = TestRunner {
            recursion: 0,
            out_stream: StandardStream::stdout(ColorChoice::Auto),
            colors: HashMap::new(),
        };
        runner.insert_colors();
        runner
    }

    fn print_result(&mut self, test: &dyn Test, run_result: &TestRunResult) {
        let mut padding = String::new();
        for _ in 0..self.recursion {
            padding.push_str("  ");
        }

        if let Some(color) = self.colors.get(&run_result) {
            self.out_stream.set_color(color).ok();
        }

        let result_string = match run_result {
            TestRunResult::Error(_) => "E",
            TestRunResult::Ignored => ".",
            TestRunResult::Success => "*",
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

    fn were_dependencies_successful<'a>(
        &self,
        mut dependencies: Box<dyn Iterator<Item = &'a String> + 'a>,
        previous_results: &HashMap<String, Box<dyn TestResult>>,
    ) -> bool {
        dependencies.all(|dependency| {
            previous_results.get(dependency).map_or(false, |result| {
                result.run_result() == TestRunResult::Success
            })
        })
    }

    pub fn run_suite(&mut self, suite: &mut TestSuite) -> TestSuiteResult {
        self.recursion += 1;
        let mut previous_results = HashMap::new();

        for test in suite.tests.iter_mut() {
            if self.were_dependencies_successful(test.dependencies(), &previous_results) {
                let result = test.run(self);

                self.print_result(&**test, &result.run_result());
                previous_results.insert(test.name().to_string(), result);
            } else {
                self.print_result(&**test, &TestRunResult::Ignored);
            }
        }

        self.recursion -= 1;
        TestSuiteResult::new(
            suite,
            previous_results.into_iter().map(|(_, v)| v).collect(),
        )
    }

    pub fn run_case(&mut self, case: &TestCase) -> TestCaseResult {
        let old_hook = panic::take_hook();

        let error_info: Arc<Mutex<Option<ErrorInfo>>> = Arc::new(Mutex::new(None));
        {
            let error_info = error_info.clone();
            panic::set_hook(Box::new(move |panic_info| {
                let maybe_message = panic_info.payload().downcast_ref::<&str>();
                let message = match maybe_message {
                    Some(message) => message,
                    None => "-------",
                };
                *error_info.lock().unwrap() = Some(ErrorInfo {
                    back_trace: Backtrace::new(),
                    message: String::from(message),
                });
            }));
        }

        // Error will be collected by panic hook
        panic::catch_unwind(case.test_function).ok();

        panic::set_hook(old_hook);

        let info = error_info.lock().unwrap();
        TestCaseResult::new(
            case,
            match &*info {
                Some(err) => TestRunResult::Error(Some(err.clone())),
                None => TestRunResult::Success,
            },
        )
    }
}
