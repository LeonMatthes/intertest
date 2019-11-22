use crate::test::{Test, TestResult};
use graphlib::{Graph, VertexId};

pub struct TestSuite {
    tests: Graph<Box<dyn Test>>,
    name: String,
    dependencies: Vec<String>,
    result: TestResult,
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
    { $name:ident : $($test:expr)* } => {
        child_suite! ($name[] {
            $($test;)*
        })
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

impl TestSuite {
    pub fn new(name: String) -> TestSuite {
        TestSuite::new_with_dependencies(name, Vec::new())
    }

    pub fn new_with_dependencies(name: String, dependencies: Vec<String>) -> TestSuite {
        TestSuite {
            tests: Graph::new(),
            name,
            dependencies,
            result: TestResult::NotRun,
        }
    }

    pub fn add_test(&mut self, test: Box<dyn Test>) {
        let dependencies = test
            .dependencies()
            .map(|dependency_name| {
                self.find_by_name(dependency_name).unwrap_or_else(|| {
                    panic!(
                        "Dependency '{}' for test '{}' could not be found in suite '{}'",
                        dependency_name,
                        test.name(),
                        self.name()
                    )
                })
            })
            .collect::<Vec<VertexId>>();
        let test_id = self.tests.add_vertex(test);
        for dependency in dependencies {
            self.tests.add_edge(&dependency, &test_id).unwrap();
        }
    }

    fn find_by_name(&self, name: &str) -> Option<VertexId> {
        for vertex_id in self.tests.vertices() {
            if let Some(vertex) = self.tests.fetch(vertex_id) {
                if vertex.name() == name {
                    return Some(vertex_id.clone());
                }
            }
        }
        None
    }

    fn check_dependencies(&self, test: &VertexId) -> bool {
        self.tests
            .in_neighbors(test)
            .all(|dependent_id| match self.tests.fetch(dependent_id) {
                None => false,
                Some(dependency) => *dependency.result() == TestResult::Success,
            })
    }
}

impl Test for TestSuite {
    fn name(&self) -> &str {
        &self.name
    }

    fn dependencies<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(self.dependencies.iter())
    }

    fn result(&self) -> &TestResult {
        &self.result
    }

    fn run(&mut self) -> &TestResult {
        if !self.tests.is_cyclic() {
            self.result = TestResult::Success;
            let topo = self
                .tests
                .topo()
                .map(VertexId::clone)
                .collect::<Vec<VertexId>>();

            for test_index in &topo {
                if self.check_dependencies(test_index) {
                    if let Some(test) = self.tests.fetch_mut(test_index) {
                        match test.run() {
                            TestResult::Success => {}
                            _ => self.result = TestResult::Error,
                        }
                    }
                }
            }
        }
        &self.result
    }
}
