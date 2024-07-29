use std::{collections::HashMap, time::Duration};

pub struct BenchmarkManager {
    tests: HashMap<String, Vec<Duration>>,
}

impl Default for BenchmarkManager {
    fn default() -> Self {
        Self {
            tests: HashMap::new(),
        }
    }
}

impl BenchmarkManager {
    pub fn bench(function_name: String, c: impl FnOnce()) {}
}
