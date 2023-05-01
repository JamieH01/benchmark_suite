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
        
    }
}
