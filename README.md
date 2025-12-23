# Samaritan
This repository contains Implementation of Samaritan Multilinear Polynomial Commitment Scheme in the MLPCS folder.

# Testing functionality
To run the functionaliy test of Samaritan MLPCS, please run:
```
cd MLPCS
cargo test --release
```

# Run benchmarks
To run the benchmark for 'commit', 'evaluation prove', 'evaluation verify' of Samaritan MLPCS, please run:
```
cd MLPCS
RUSTFLAGS="-C target_cpu=native" cargo bench --bench samaritan_mlpcs_bench
```
