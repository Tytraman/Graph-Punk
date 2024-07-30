use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

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
    pub fn bench(&mut self, function_name: String, c: impl FnOnce()) {
        let map = match self.tests.get_mut(&function_name) {
            Some(value) => value,
            None => {
                self.tests.insert(function_name.clone(), Vec::new());
                self.tests.get_mut(&function_name).unwrap()
            }
        };

        let instant = Instant::now();

        (c)();

        map.push(instant.elapsed());
    }

    pub fn print_results(&self) {
        self.tests.iter().for_each(|(name, durations)| {
            print!("Benchmark for '{name}': ");

            let mut total = 0;

            durations
                .iter()
                .for_each(|duration| total += duration.as_micros());

            total /= durations.len() as u128;

            println!("{total} micros average");
        });
    }
}
