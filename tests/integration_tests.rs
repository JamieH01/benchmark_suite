mod integration_tests {
    //use std::{thread, time::Duration};

    use benchmark_suite::*;

    #[test]
    fn it_works() {
        struct Table {}

        impl Bench for Table {
            fn generate() -> Self {
                Table {}
            }
    
            fn test(&mut self) {
                let _dummy =vec![15; 10_000_000];
            }
        }
        

        quickbench!(bench, Table, 8, 10);
        

        quickbench!(define TestSruct; {
            let _dummy =vec![15; 10_000_000];
        }, 8, 10);

    }
    #[test]
    fn sorter_test() {
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
    
        let mut bencher = quickbench!(Sorter, 8, 100);
        bencher.start();
        println!(
            "{:?} / {}",
            bencher.average(),
            bencher.max_threads * bencher.max_runcount,
        )

    }
}
