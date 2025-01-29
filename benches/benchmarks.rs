mod bench {
    use diol::prelude::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::hint::black_box;

    static TRIGRAMS: &[&str] = &["the", "dof", "smi", "wha", "hwy", "lma"];

    pub(super) fn main() -> std::io::Result<()> {
        let mut bench = Bench::new(BenchConfig::from_args()?);

        bench.register_many(list![get_trigrams_json, get_trigrams_sql], [0, 1, 6]);

        bench.run()?;

        Ok(())
    }

    fn get_trigrams_json(bencher: Bencher, amount: usize) {
        let trigrams = black_box(&TRIGRAMS[..amount]);

        bencher.bench(|| {
            let _ = old_benchmarks::get_trigrams_json(trigrams);
        })
    }

    fn get_trigrams_sql(bencher: Bencher, amount: usize) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let database_url = "./test.db";

        let pool = rt
            .block_on(
                SqlitePoolOptions::new()
                    .max_connections(5)
                    .acquire_timeout(std::time::Duration::from_secs(3))
                    .connect(&database_url),
            )
            .expect("Couldn't connect to database");

        let trigrams = black_box(&TRIGRAMS[..amount]);

        bencher.bench(|| {
            let _ = rt.block_on(black_box(rmini_core::freqs::get_trigrams(
                &pool, "english", trigrams,
            )));
        })
    }
}

fn main() -> std::io::Result<()> {
    bench::main()
}
