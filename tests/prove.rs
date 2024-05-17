use halo2_proofs::{circuit::Value, dev::MockProver, halo2curves::bn256::Fr as Fp};
use poc_underconstrained_halo2::circuits::dummy_circuit::DummyCircuit;

#[test]
fn test_pass() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(3)),
        b: Value::known(Fp::from(4)),
    };
    let out = Fp::from(7);

    let k = 4;
    let prover = MockProver::run(k, &circuit, vec![vec![out]]).unwrap();
    assert!(prover.verify().is_ok());
}

#[test]
fn test_fail_out_of_range() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(5)),
        b: Value::known(Fp::from(4)),
    };
    let out = Fp::from(9);

    let k = 4;
    let prover = MockProver::run(k, &circuit, vec![vec![out]]).unwrap();
    println!("{:?}", prover.verify());
    assert!(prover.verify().is_err());
}

#[test]
fn test_should_fail_underconstrained() {
    let circuit = DummyCircuit {
        a: Value::known(Fp::from(0)),
        b: Value::known(Fp::from(4)),
    };
    let out = Fp::from(4);

    let k = 4;
    let prover = MockProver::run(k, &circuit, vec![vec![out]]).unwrap();
    println!("{prover:?}");
    println!("{:?}", prover.verify());

    // here we should have an error: assert!(prover.verify().is_err());
    // but the verification passes
    assert!(prover.verify().is_ok());
}
