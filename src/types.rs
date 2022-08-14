use serde::{Deserialize, Serialize};
use bls12_381::{G1Affine, G2Affine, G2Prepared};

#[derive(Serialize, Deserialize)]
pub struct ProofStr {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    pub protocol: String,
    pub curve: String,
}

#[derive(Serialize, Deserialize)]
pub struct PvkStr {
    pub protocol: String,
    pub curve: String,
    pub nPublic: u8,
    pub vk_alpha_1: Vec<String>,
    pub vk_beta_2: Vec<Vec<String>>,
    pub vk_gamma_2: Vec<Vec<String>>,
    pub vk_delta_2: Vec<Vec<String>>,
    pub vk_alphabeta_12: Vec<Vec<Vec<String>>>,
    pub IC: Vec<Vec<String>>,
}