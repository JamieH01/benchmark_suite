//!This crate allows for fast benchmarking of both complex and simple structs/closures, with complete control over I/O.
//!The BenchMarker struct can be created from any type that implements the [Bench] trait, which lets you dictate input generation and the actual test to be ran.
//!Here is an example of a sorting algorithm implementation with the trait:
//!```rust
//!use rand::prelude::*;
//!use benchmark_suite::*;
//!
//!struct Sorter {
//!    table:Vec<u32>
//!}
//!
//!impl Bench for Sorter {
//!
//!    fn generate() -> Self {
//!        let mut rng = rand::thread_rng();
//!
//!        let mut table:Vec<u32> = (1..100).collect();
//!        table.shuffle(&mut rng);
//!        Sorter {table}
//!    }
//!    fn test(&mut self) {
//!        let mut swapped = true;
//!
//!        while swapped {
//!            swapped = false;
//!
//!            for i in 0..self.table.len()-1 {
//!                let a = self.table[i];
//!                let b = self.table[i+1];
//!
//!                if a > b {
//!                    swapped = true;
//!                    self.table[i] = b;
//!                    self.table[i+1] = a;
//!                }
//!            }
//!        
//!        }
//!    }
//!
//!}
//!```
//!After that, initialize, run, and display a benchmarking test with the [quickbench]! macro or manually:
//!```rust
//!//note that the total number of tests ran will be threads * runcount
//!
//!//manually
//!let mut benchmark = BenchMarker::<Sorter>::new(8, 50);
//!benchmark.start(); 
//!println!("{}", benchmark);
//! 
//!//with macro
//!//var name, struct, threads, runcount
//!quickbench!(benchmark, Sorter, 8, 50);
//!```

use indicatif::ProgressBar;
use std::any::type_name;
use std::marker::PhantomData;
use std::thread;
use std::time::{Duration, Instant};
use std::fmt::Debug;
use num::integer::Roots;
use owo_colors::OwoColorize;



#[macro_export] macro_rules! quickbench {
    ($struct:ty, $threads:expr, $run:expr) => {
        BenchMarker::<$struct>::new($threads, $run)
    };
    
    ($name:ident, $struct:ty, $threads:expr, $run:expr) => {
        let mut $name = BenchMarker::<$struct>::new($threads, $run);
        $name.start();
        println!("{}", $name);
    };

}

type DC = DisplayCfg;
///default display config.
///```
///SysInfo
///
///Mean
///Median
///Deviation
/// 
///Min
///Max
///Diff
///```
pub const DEFAULT:[DisplayCfg; 9] = 
[DC::SysInfo, DC::Space, 
DC::Mean, DC::Median, DC::Deviation, DC::Space,
DC::AbsMin, DC::AbsMax, DC::AbsDiff]; 

///The main benchmarking struct that manages running and storing tests.
pub struct BenchMarker<T:Bench> {
    phantom:PhantomData<T>,
    time_table: Vec<Duration>,
    //_thread_table: Vec<ScopedJoinHandle<'scope, Duration>>,
    pub max_threads: usize,
    pub max_runcount: usize,
    runtime:Duration,
    pub display_config:Vec<DisplayCfg>
}

impl<T:Bench> BenchMarker<T> {
    ///Returns a new [BenchMarker].
    pub fn new(max_threads: usize, max_runcount: usize) -> Self {
        Self {
            phantom:PhantomData,
            time_table: vec![],
            //_thread_table: vec![],
            max_threads,
            max_runcount,
            runtime:Duration::ZERO,
            display_config:DEFAULT.to_vec()
        }
    }

    ///Begins the specified number of tests and pushes results to the internal time table. Running again will continue to push new results and make data more accurate.
    pub fn start(&mut self) {
        
        let bar = ProgressBar::new((self.max_runcount * self.max_threads) as u64);
        bar.set_position(1);
        let runtime = Instant::now();
        thread::scope(|s| {
            for _ in 0..self.max_runcount {
            let mut scope_table = vec![];   
            
            for _ in 0..self.max_threads {
                scope_table.push(s.spawn(|| {
                    let mut item = <T as Bench>::generate();
                    let time = Instant::now();
                    item.test();
                    time.elapsed()
                }));
                bar.inc(1);
            }

            for _ in 0..self.max_threads {
                self.time_table.push(scope_table.pop().expect("Error joining scopes").join().expect("Error joining scopes"));
            }
            
        }
        });
        self.time_table.sort();
        self.runtime += runtime.elapsed();
        bar.finish();

    }

}

impl<T:Bench + Debug> BenchMarker<T> {
    ///Runs a single test and prints its inputs, outputs, and runtime.
    pub fn debug(&mut self) {
        let mut item = <T as Bench>::generate();
        let name = type_name::<T>().split("::").last().unwrap_or("[parse_err]");
        println!("Test Results for {}", name.green());

        println!("{:?}", item.red());
        let time = Instant::now();
        item.test();
        println!("{:?}\n{:?}", item.blue(), time.elapsed().yellow());
    }
}

///formats benchmarking results based on the configurable `display_config` field. See enum [DisplayCfg].
impl<T:Bench> std::fmt::Display for BenchMarker<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<T>().split("::").last().unwrap_or("[parse_err]");
        //mean
        let mut sum = Duration::ZERO;
        self.time_table.iter().for_each(|i| sum += *i);
        let mean = sum / self.time_table.len() as u32;



        //deviation
        let mut sum_table:Vec<i128> = vec![];
        self.time_table.iter().for_each(|i| {sum_table.push(i.as_nanos() as i128 - mean.as_nanos() as i128)});
        let sum:i128 = sum_table.iter().sum();
        let i_dev = sum.pow(2) / self.time_table.len() as i128;
        let deviation = Duration::from_nanos(i_dev.sqrt().try_into().unwrap_or(0));

        //quartiles
        let q1 = self.time_table[self.time_table.len()/4];
        let median = self.time_table[self.time_table.len()/2];
        let q3 = self.time_table[self.time_table.len()-(self.time_table.len()/4)];
        let iqr = q3 - q1;

        //range
        let first = *self.time_table.first().unwrap_or(&Duration::ZERO);
        let q_min = q1-(iqr + (iqr/2));
        let last = *self.time_table.last().unwrap_or(&Duration::ZERO);
        let q_max = q3-(iqr + (iqr/2));
        let range = last - first;

        writeln!(f, "Benchmark Results for {}", name.green())?;

        for item in self.display_config.iter() {
            match item {
                DisplayCfg::SysInfo => {
                writeln!(f, "{}", format!("    threads used: {:?}", self.max_threads).cyan())?;
                writeln!(f, "{}", format!("    total tests ran: {:?}", self.max_threads * self.max_runcount).cyan())?;
                writeln!(f, "{}", format!("    total runtime: {:?}\n", self.runtime).cyan())?;
                },
                DisplayCfg::Mean => {
                    writeln!(f, "    mean: {:?}", mean.yellow())?;
                },
                DisplayCfg::Median => {
                    writeln!(f, "    median: {:?}", median)?;
                },
                DisplayCfg::Quartiles => {
                    writeln!(f, "    Q1: {:?}", q1)?;
                    writeln!(f, "    Q2: {:?}", median)?;
                    writeln!(f, "    Q3: {:?}", q3)?;
                },
                DisplayCfg::Deviation => {
                    writeln!(f, "    deviation: {:?}\n", deviation.magenta())?;
                },
                DisplayCfg::AbsMin => {
                    writeln!(f, "    min: {:?}", first)?;
                },
                DisplayCfg::QuartileMin => {
                    writeln!(f, "    quartile min: {:?}", q_min)?;
                },
                DisplayCfg::AbsMax => {
                    writeln!(f, "    max: {:?}", last)?;
                },
                DisplayCfg::QuartileMax => {
                    writeln!(f, "    quartile max: {:?}", q_max)?;
                },
                DisplayCfg::AbsDiff => {
                    writeln!(f, "    diff: {:?}", range.magenta())?;
                },
                DisplayCfg::Space => {
                    writeln!(f)?;
                },
            }
        }

        writeln!(f)

        //color code
        //white-direct element of data
        //green-struct name
        //cyan-system info
        //yellow-information derived from data meant to represent a sample, not inside the data itself
        //magenta-information derived from data meant to give insight on the structure and distribution of data
    }
}


///The types of information a benchmark can display. The internal `display_config` vector dictates what is shown in what order. Change this to your hearts content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayCfg {
    SysInfo,
    Mean,
    Median,
    Quartiles,
    Deviation,
    AbsMin,
    QuartileMin,
    AbsMax,
    QuartileMax,
    AbsDiff,
    Space
}


///The trait to implement for benchmarking.
///`generate()` is how the struct should be generated; `test()` will be called on it.
///```rust
/// //example if you have a very simple struct or just need to test a closure
///struct Table {table:Vec<i32>}
///impl Bench for Table {
///    fn generate() -> Self {
///        Table {table:vec![]}
///    }
///    fn test(&mut self) {
///        self.table = vec![15; 10_000_000];
///    }
///}
///```
pub trait Bench {
    fn generate() -> Self;
    fn test(&mut self);
}

