use bellman::{Circuit, ConstraintSystem, SynthesisError, groth16};
use bls12_381::{Bls12, Scalar as Fr};
use rand::rngs::OsRng;

#[derive(Clone)]
pub struct SimpleCircuit {
    pub value: Option<Fr>,
}
impl Circuit<Fr> for SimpleCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let _value_var = cs.alloc(|| "value", || self.value.ok_or(SynthesisError::AssignmentMissing))?;
        
        cs.enforce(
	    || "constraint _value_var",
            |lc| lc + _value_var,
	    |lc| lc + CS::one(),
	    |lc| lc + _value_var
	);

        Ok(())
    }
}

pub fn generate_proof(circuit: SimpleCircuit, params: &groth16::Parameters<Bls12>) -> Result<groth16::Proof<Bls12>, SynthesisError> {
    let rng = &mut OsRng::default();
    let proof = groth16::create_random_proof(circuit, params, rng)?;
    Ok(proof)
}

pub fn verify_proof(proof: groth16::Proof<Bls12>, params: &groth16::Parameters<Bls12>) -> bool {
    let pvk = groth16::prepare_verifying_key(&params.vk);
    groth16::verify_proof(&pvk, &proof, &[]).is_ok()
}
