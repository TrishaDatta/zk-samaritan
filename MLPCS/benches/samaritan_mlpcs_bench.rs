use ark_bls12_381::Bls12_381;
use ark_bls12_381::Fr;
// use ark_bn254::Bn254;
// use ark_bn254::Fr;
use ark_std::test_rng;
use ark_std::{vec::Vec};
use ark_std::UniformRand;
use ark_poly::polynomial::Polynomial;
use ark_poly::evaluations::multivariate::multilinear::MultilinearExtension;
use ark_poly::evaluations::multivariate::multilinear::DenseMultilinearExtension;

use criterion::*;

use MLPCS::samaritan_mlpcs::*;

type SamaritanMLPCS_Bls12_381 = SamaritanMLPCS<Bls12_381>;

// type SamaritanMLPCS_Bn254 = SamaritanMLPCS<Bn254>;

fn commit_benchmark(c: &mut Criterion) {
  for num_vars in (10..=24).step_by(2) {
    let mut group = c.benchmark_group("Samaritan_MLPCS_commit_benchmark");

    let mut rng = &mut test_rng();

    let mlp = DenseMultilinearExtension::rand(num_vars, rng);

    let srs = SamaritanMLPCS_Bls12_381::hiding_setup(num_vars, &mut rng).unwrap();

    let mlp_evals = mlp.to_evaluations();
    
    let name = format!("Samaritan_MLPCS_commit_{}", num_vars);

    let r_f = Fr::rand(rng);

    group.bench_function(&name, move |b| {
      b.iter(|| {
        SamaritanMLPCS_Bls12_381::zk_commit_G1(&srs, mlp_evals.clone(), r_f).unwrap();
      });
    });
    group.finish();
  }
}

fn eval_prove_benchmark(c: &mut Criterion) {
  for num_vars in (10..=24).step_by(2) {
    let mut group = c.benchmark_group("Samaritan_MLPCS_eval_prove_benchmark");

    // let n: usize = 1 << num_vars;
    let mut rng = &mut test_rng();

    let mlp = DenseMultilinearExtension::rand(num_vars, rng);
    let mlp_evals = mlp.to_evaluations();

    let srs = SamaritanMLPCS_Bls12_381::hiding_setup(num_vars, &mut rng).unwrap();

    let point: Vec<_> = (0..num_vars).map(|_| Fr::rand(rng)).collect();

    let eval = mlp.evaluate(&point);

    let r_f = Fr::rand(rng);
    
    let name = format!("Samaritan_MLPCS_eval_prove_{}", num_vars);

    group.bench_function(&name, move |b| {
      b.iter(|| {
        SamaritanMLPCS_Bls12_381::zk_prove(&srs, &mlp, &point, eval, r_f, mlp_evals.clone()).unwrap();
      });
    });
    group.finish();
  }
}

fn eval_verify_benchmark(c: &mut Criterion) {
  for num_vars in (10..=24).step_by(2) {
    let mut group = c.benchmark_group("Samaritan_MLPCS_eval_verify_benchmark");

    // let n: usize = 1 << (num_vars - 1);
    let mut rng = &mut test_rng();

    let mlp = DenseMultilinearExtension::rand(num_vars, rng);
    let mlp_evals = mlp.to_evaluations();

    let srs = SamaritanMLPCS_Bls12_381::hiding_setup(num_vars, &mut rng).unwrap();

    let r_f = Fr::rand(rng);

    let comm = SamaritanMLPCS_Bls12_381::zk_commit_G1(&srs, mlp_evals.clone(), r_f).unwrap();

    let point: Vec<_> = (0..num_vars).map(|_| Fr::rand(rng)).collect();

    let eval = mlp.evaluate(&point);


    let eval_proof = SamaritanMLPCS_Bls12_381::zk_prove(&srs, &mlp, &point, eval, r_f, mlp_evals).unwrap();

    let name = format!("Samaritan_MLPCS_eval_verify_{}", num_vars);

    group.bench_function(&name, move |b| {
      b.iter(|| {
        SamaritanMLPCS_Bls12_381::zk_verify(&srs, &comm, &point, eval, &eval_proof).unwrap();
      });
    });
    group.finish();
  }
}

criterion_group! {
    name = hybridplonk_benches;
    config = Criterion::default().sample_size(10);
    targets = commit_benchmark, eval_prove_benchmark, eval_verify_benchmark
}
criterion_main!(hybridplonk_benches);