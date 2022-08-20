use crate::types::{ProofStr, PvkStr};
use bls12_381::{Bls12, Gt, G1Affine, G2Affine, G2Prepared};

// convert the vec to Gt 

fn parseVecToGt(v: &Vec<String>) -> Gt{
    Gt::generator()
}

// convert the vec to G2Prepared 

fn parseVecToG2Prepared(v: &Vec<String>) -> G2Prepared{
    G2Affine::generator().to_prepared()
}

// convert the vec to G1Affine 

fn parseVecToG1Affine(v: &Vec<String>) -> G1Affine{
    G1Affine::generator()
}

// convert the vec to G2Affine 

fn parseVecToG2Affine(v: &Vec<String>) -> G2Affine{
    G2Affine::generator()
}