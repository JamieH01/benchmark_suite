mod integration_tests {
    //use std::{thread, time::Duration};

    use benchmark_suite::*;

    #[test]
    fn it_works() {
        #[derive(Debug)]
        struct Table {table:Vec<i32>}

        impl Bench for Table {
            fn generate() -> Self {
                Table {table:vec![]}
            }
    
            fn test(&mut self) {
                self.table = vec![15; 10_000_000];
            }
        }
        

        quickbench!(bench, Table, 8, 10);
        
        


    }



#[test]
fn sorter_test() {
    const SIZE:u32 = 1000;

    //these wont work btw
    use rand::prelude::*;
    use benchmark_suite::*;
    #[derive(Debug)]
    struct Comb {
        table:Vec<u32>
    }

    impl Bench for Comb {
        fn generate() -> Self {
            let mut rng = rand::thread_rng();
            let mut table:Vec<u32> = (1..SIZE).collect();
            table.shuffle(&mut rng);
            Comb {table}
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



    quickbench!(comb, Comb, 8, 5);
    comb.debug()

    }
}
