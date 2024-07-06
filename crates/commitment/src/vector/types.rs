use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

// Commitment for a vector of field elements.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    config: Config,
    #[serde_as(as = "UfeHex")]
    commitment_hash: Felt,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde_as(as = "UfeHex")]
    height: Felt,
    #[serde_as(as = "UfeHex")]
    n_verifier_friendly_commitment_layers: Felt,
}

// A query to the vector commitment.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Query {
    #[serde_as(as = "UfeHex")]
    pub index: Felt,
    #[serde_as(as = "UfeHex")]
    pub value: Felt,
}

// A query to the vector commitment that contains also the depth of the query in the Merkle tree.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryWithDepth {
    #[serde_as(as = "UfeHex")]
    pub index: Felt,
    #[serde_as(as = "UfeHex")]
    pub value: Felt,
    #[serde_as(as = "UfeHex")]
    pub depth: Felt,
}

// Witness for a decommitment over queries.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    // The authentication values: all the siblings of the subtree generated by the queried indices,
    // bottom layer up, left to right.
    #[serde_as(as = "Vec<UfeHex>")]
    authentications: Vec<Felt>,
}
