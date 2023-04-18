use std::any::Any;
use std::thread::{self, ScopedJoinHandle};
use std::time::{Duration, Instant};

//trait AnySized: Any + ?Sized {}

//#[allow(dead_code)]
pub struct BenchMarker<'scope, F, T>
where
    F: Fn() -> T + Sync + Send,
    T: Any,
{
    pub code: F,
    time_table: Vec<Duration>,
    _thread_table: Vec<ScopedJoinHandle<'scope, Duration>>,
    pub max_threads: usize,
    pub max_runcount: usize,
}

impl<'scope, F: Fn() -> T + Sync + Send, T: Any> BenchMarker<'_, F, T> {
    pub fn new(code: F, max_threads: usize, max_runcount: usize) -> Self {
        Self {
            code,
            time_table: vec![],
            _thread_table: vec![],
            max_threads,
            max_runcount,
        }
    }

    fn _run_code(&mut self) {
        let time = Instant::now();
        (self.code)();
        self.time_table.push(time.elapsed());
    }

    pub fn start(&mut self) {
        thread::scope(|s| {
            for _ in 0..self.max_runcount {
                for _ in 0..self.max_threads {
                    let t = s.spawn(|| {
                        let time = Instant::now();
                        (self.code)();
                        time.elapsed()
                    });
                    self.time_table.push(t.join().unwrap());
                }
            }
        });
    }

    pub fn average(&self) -> Duration {
        let mut sum = Duration::ZERO;
        self.time_table.iter().map(|i| sum += *i).collect::<()>();
        sum / self.time_table.len() as u32
    }
}
