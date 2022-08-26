// For randomness (during paramgen and proof generation)
use rand::thread_rng;

// Bring in some tools for using finite fiels
use ff::Field;

// We're going to use the BLS12-381 pairing-friendly elliptic curve.
use bls12_381::{Bls12, Scalar, G1Affine};

// We're going to use the Groth16 proving system.
use bellman::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, PreparedVerifyingKey, VerifyingKey};
use pairing::{MillerLoopResult, MultiMillerLoop};

mod circuit;
use circuit::*;

mod snark;
use snark::snark_proof_bellman_verify;

fn main() {
    let mut rng = thread_rng();

    // Generate the MiMC round constants
    let constants = (0..MIMC_ROUNDS)
        .map(|_| Scalar::random(&mut rng))
        .collect::<Vec<_>>();

    println!("Creating parameters...");

    // Create parameters for our circuit
    let params = {
        let c = MiMCDemo {
            xl: None,
            xr: None,
            constants: &constants,
        };

        generate_random_parameters::<Bls12, _, _>(c, &mut rng).unwrap()
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("ic 为: ");
    println!("{:?}", &params.vk.ic[0].to_uncompressed());
    println!("{:?}", &params.vk.ic[1].to_uncompressed());


    // test uncompressed function Affine => [u8, 96]
    println!("{:?}", &params.vk.alpha_g1);
    println!("{:?}", &params.vk.alpha_g1.to_uncompressed());

    let compressed_alpha = &params.vk.alpha_g1.to_uncompressed();
    let affine_alpha = G1Affine::from_uncompressed(compressed_alpha);
    println!("{:?}", affine_alpha);
    // CtOption { value: G1Affine { x: 0x09acb8b985e88cb899233dd730802178430d871262342f1873e49be633dc204fcb7da433eb7c763bf8760823919fb692, y: 0x04a8536c21d6fa01068a70fa5e9342707bd8d30704a5f3964482391628ba65276179d785b06c7392cdc8c7ca417f0470, infinity: Choice(0) }, is_some: Choice(1) }


    println!("Creating proofs...");
    let xl = Scalar::random(&mut rng);
    let xr = Scalar::random(&mut rng);
    
    let image = mimc(xl, xr, &constants);

    // Create an instance of our circuit (with the
    // witness)
    let c = MiMCDemo {
        xl: Some(xl),
        xr: Some(xr),
        constants: &constants,
    };

    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, &mut rng).unwrap();

    // println!("proof b is ");
    // println!("{:?}", proof.b.into());

    assert!(verify_proof(&pvk, &proof, &[image]).is_ok());


    snark_proof_bellman_verify()

}
