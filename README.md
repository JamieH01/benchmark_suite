Quickly and easily benchmark code, with complete control over I/O generation and built-in multithreading.

# Quickstart
To use Benchmarker, your input data must be inside of a struct that implements `Bench`. This allows complete control over how your inputs are generated for each test (i.e. using rand).
```rust
use rand::prelude::*;
use benchmark_suite::*;

struct Sorter {
    table:Vec<u32>
}

impl Bench for Sorter {

    fn generate() -> Self {
        let mut rng = rand::thread_rng();

        let mut table:Vec<u32> = (1..100).collect();
        table.shuffle(&mut rng);
        Sorter {table}
    }
    fn test(&mut self) {
        let mut swapped = true;

        while swapped {
            swapped = false;

            for i in 0..self.table.len()-1 {
                let a = self.table[i];
                let b = self.table[i+1];

                if a > b {
                    swapped = true;
                    self.table[i] = b;
                    self.table[i+1] = a;
                }
            }
        
        }
    }

}
```
Now, you can create the benchmarker with the `quickbench!` macro, and run a test with `start()`. Starting another test after will continue to add to the list of times, making them more detailed.
```rust
//note that the runcount is the amount of times threads are created, the total number of tests ran is threads * runcount

                            //struct, threads, runcount
let mut bencher = quickbench!(Sorter, 8, 100);
bencher.start();
println!(
    "{:?} / {}",
    bencher.average(),
    bencher.max_threads * bencher.max_runcount,
)
```
