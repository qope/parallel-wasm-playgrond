use plonky2::{
    field::{
        goldilocks_field::GoldilocksField,
        types::{Field, Sample},
    },
    hash::{
        merkle_proofs::{verify_merkle_proof_to_cap, MerkleProofTarget},
        merkle_tree::MerkleTree,
        poseidon::PoseidonHash,
    },
    iop::{
        target::Target,
        witness::{PartialWitness, WitnessWrite},
    },
    plonk::{
        circuit_builder::CircuitBuilder, circuit_data::CircuitConfig,
        config::PoseidonGoldilocksConfig,
    },
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn merkle_tree() -> String {
    const D: usize = 2;
    let config = CircuitConfig::standard_recursion_config();
    type C = PoseidonGoldilocksConfig;
    type F = GoldilocksField;
    let tree_height = 10;
    let n = 1 << tree_height;
    let leaves: Vec<Vec<F>> = (0..n).map(|_| F::rand_vec(4)).collect();
    let tree = MerkleTree::<F, PoseidonHash>::new(leaves.clone(), 0);
    let leaf_index = 3;
    let leaf = tree.get(leaf_index);
    let root = tree.cap.0[0];
    let proof = tree.prove(leaf_index);
    let _ = verify_merkle_proof_to_cap(leaf.to_vec(), leaf_index, &tree.cap, &proof);

    let mut builder = CircuitBuilder::<F, D>::new(config.clone());
    let proof_t = MerkleProofTarget {
        siblings: builder.add_virtual_hashes(tree_height),
    };
    let leaf_index_t = builder.add_virtual_target();
    let root_t = builder.add_virtual_hash();
    builder.register_public_inputs(&root_t.elements);
    let leaf_t: [Target; 4] = builder.add_virtual_targets(4).try_into().unwrap();
    let leaf_index_bits_t = builder.split_le(leaf_index_t, tree_height);
    builder.verify_merkle_proof::<PoseidonHash>(
        leaf_t.to_vec(),
        &leaf_index_bits_t,
        root_t,
        &proof_t,
    );
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(leaf_index_t, F::from_canonical_usize(leaf_index));
    for i in 0..4 {
        pw.set_target(leaf_t[i], leaf[i]);
    }
    pw.set_hash_target(root_t, root);
    for i in 0..proof.siblings.len() {
        pw.set_hash_target(proof_t.siblings[i], proof.siblings[i]);
    }
    let data = builder.build::<C>();
    let proof = data.prove(pw).unwrap();
    let proof_str = serde_json::to_string(&proof).unwrap();
    dbg!(data.common.degree_bits());
    proof_str
}
