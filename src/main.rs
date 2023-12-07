mod zkp;
use zkp::{SimpleCircuit, generate_proof, verify_proof};
use bls12_381::{Bls12, Scalar as Fr};
use bellman::groth16;
use rand::rngs::OsRng;

fn main() {
    let circuit = SimpleCircuit { value: Some(Fr::one()) };

    let rng = &mut OsRng::default();
    let params = match groth16::generate_random_parameters::<Bls12, _, _>(circuit.clone(), rng) {
        Ok(params) => params,
        Err(e) => {
            println!("Error generating parameters: {:?}", e);
            return;
        }
    };

    match generate_proof(circuit.clone(), &params) {
        Ok(proof) => {
            println!("Proof generated successfully!");

            if verify_proof(proof, &params) {
                println!("Proof verified successfully!");
            } else {
                println!("Proof verification failed.");
            }
        },
        Err(e) => println!("Error generating proof: {:?}", e),
    }
}
