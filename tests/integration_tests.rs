mod integration_tests {
    use benchmark_suite::*;

    #[test]
    fn it_works() {

        let mut bench = BenchMarker::new(|| vec![15_usize; 1000000], 5, 20);
        bench.start();
        println!(
            "{:?} / {}",
            bench.average(),
            bench.max_threads * bench.max_runcount
        )
    }
}
