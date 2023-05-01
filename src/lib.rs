#![warn(clippy::unwrap_used)]

use std::marker::PhantomData;
use std::thread;
use std::time::{Duration, Instant};
use std::hint::black_box;
//use rand::prelude::*;

#[macro_export] macro_rules! quickbench {
    ($struct:ty, $threads:expr, $run:expr) => {
        BenchMarker::<$struct>::new($threads, $run)
    };
    
    ($name:ident, $struct:ty, $threads:expr, $run:expr) => {
        let mut $name = BenchMarker::<$struct>::new($threads, $run);
        $name.start();
        println!(
            "{:?} / {}",
            $name.average(),
            $name.max_threads * $name.max_runcount,
        )
    };

    (define $name:ident; $code:block, $threads:expr, $run:expr) => {
        struct $name {}

        impl Bench for $name {
            fn generate() -> Self {
                $name {}
            }

            fn test(&mut self) {
                $code
            }
        }
        #[allow(non_snake_case)]
        let mut $name = BenchMarker::<$name>::new($threads, $run);
        $name.start();
        println!(
            "{:?} / {}",
            $name.average(),
            $name.max_threads * $name.max_runcount,
        )
    };
}



pub struct BenchMarker<T:Bench> {
    phantom:PhantomData<T>,
    time_table: Vec<Duration>,
    //_thread_table: Vec<ScopedJoinHandle<'scope, Duration>>,
    pub max_threads: usize,
    pub max_runcount: usize,
}

impl<T:Bench> BenchMarker<T> {
    pub fn new(max_threads: usize, max_runcount: usize) -> Self {
        Self {
            phantom:PhantomData,
            time_table: vec![],
            //_thread_table: vec![],
            max_threads,
            max_runcount,
        }
    }


    pub fn start(&mut self) {
        thread::scope(|s| {
            for _ in 0..self.max_runcount {
            let mut scope_table = vec![];   
            
            for _ in 0..self.max_threads {
                scope_table.push(s.spawn(|| {black_box({
                    let mut item = <T as Bench>::generate();
                    let time = Instant::now();
                    item.test();
                    time.elapsed()
                })}));
            }

            for _ in 0..self.max_threads {
                self.time_table.push(scope_table.pop().expect("Error joining scopes").join().expect("Error joining scopes"));
            }
        }
    });
    }

    pub fn average(&self) -> Duration {
        let mut sum = Duration::ZERO;
        self.time_table.iter().for_each(|i| sum += *i);
        sum / self.time_table.len() as u32
    }
}






pub trait Bench {
    fn generate() -> Self;
    fn test(&mut self);
}