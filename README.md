# Samaritan
This repository contains Implementation of ZK Samaritan Multilinear Polynomial Commitment Scheme in the MLPCS folder.

# Testing functionality
To run the functionaliy test of ZK Samaritan MLPCS, please run:
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

Branched from https://github.com/SayaniSinha97/Samaritan/tree/main
