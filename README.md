# Benchmarks
generated by running:
```bash
cargo bench
```

```
    Finished bench [optimized] target(s) in 0.04s
     Running unittests src/main.rs (target/release/deps/seqspeed-60addd475ca57873)

running 6 tests
test test::speed_bio         ... bench:     432,252 ns/iter (+/- 5,716)
test test::speed_bio_gzip    ... bench:   1,111,662 ns/iter (+/- 10,146)
test test::speed_fastq       ... bench:      96,229 ns/iter (+/- 805)
test test::speed_fastq_gzip  ... bench:     772,047 ns/iter (+/- 4,024)
test test::speed_fxread      ... bench:     191,968 ns/iter (+/- 1,193)
test test::speed_fxread_gzip ... bench:     995,568 ns/iter (+/- 4,294)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out; finished in 2.10s
```
