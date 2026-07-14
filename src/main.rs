use minstark::{F, HashChain, Prover, Trace, Verifier};

fn main() {
    let seed = F::new(1);
    let inputs = vec![F::new(10), F::new(20), F::new(30), F::new(40), F::new(50)];
    println!("--- Mini STARK (Poseidon Hash Chain) ---");
    println!("seed      : {}", seed);
    println!("steps     : {}", inputs.len());
    println!(
        "inputs    : {:?}",
        inputs.iter().map(|x| x.as_int()).collect::<Vec<_>>()
    );
    let mut chain = HashChain::new(seed);
    for &value in &inputs {
        chain.append(value);
    }
    let tip = chain.tip();
    println!("final hash: {}", tip);
    let trace = Trace::from_hash_chain(&chain);
    let prover = Prover::new(trace.length);
    let proof = prover.prove(&trace, &inputs);
    println!("\nproof:");
    println!("  root   : {}", proof.root);
    println!("  length : {}", proof.length);
    let verifier = Verifier::new();
    let ok = verifier.verify(&proof, proof.root, trace.length, &trace.values, &inputs);
    println!(
        "\nverifier result: {}",
        if ok { "ACCEPTED" } else { "REJECTED" }
    );
}
