use crate::test::{Test, TestResult};
use crate::test_runner::TestRunner;
use graphlib::{Graph, VertexId};

pub struct TestSuite {
    pub tests: Graph<Box<dyn Test>>,
    name: String,
    dependencies: Vec<String>,
    result: TestResult,
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

    pub fn check_dependencies(&self, test: &VertexId) -> bool {
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

    fn run(&mut self, runner: &mut TestRunner) -> &TestResult {
        self.result = runner.run_suite(self);
        &self.result
    }
}
