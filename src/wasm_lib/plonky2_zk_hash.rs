use plonky2::{
    field::{goldilocks_field::GoldilocksField, types::Field},
    hash::poseidon::PoseidonHash,
    iop::witness::PartialWitness,
    plonk::{
        circuit_builder::CircuitBuilder, circuit_data::CircuitConfig,
        config::PoseidonGoldilocksConfig,
    },
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn zk_hash() -> String {
    const D: usize = 2;
    let config = CircuitConfig::standard_recursion_zk_config();
    type C = PoseidonGoldilocksConfig;
    type F = GoldilocksField;
    type H = PoseidonHash;
    let mut builder = CircuitBuilder::<F, D>::new(config);
    let preimage = builder.constant(F::ONE);
    let hash = builder.hash_n_to_hash_no_pad::<H>(vec![preimage]);
    hash.elements.map(|h| builder.register_public_input(h));
    let data = builder.build::<C>();
    let pw = PartialWitness::<F>::new();
    let proof = data.prove(pw).unwrap();
    let proof_str = serde_json::to_string(&proof).unwrap();
    dbg!(data.common.degree_bits());
    proof_str
}
