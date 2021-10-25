/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVShieldedSpend {
    /// Defines a Merkle tree root of a note commitment tree which uniquely identifies a note commitment tree state given the assumed security properties of the Merkle tree’s hash function.
    #[serde(rename = "anchor")]
    pub anchor: String,
    /// Defines a value commitment to the value of the input note.
    #[serde(rename = "cv")]
    pub cv: String,
    /// Represents a sequence of nullifiers of the input notes.
    #[serde(rename = "nullifier")]
    pub nullifier: String,
    /// Represents the proof.
    #[serde(rename = "proof")]
    pub proof: String,
    /// Represents the randomized validating key for spendAuthSig.
    #[serde(rename = "rk")]
    pub rk: String,
    /// Used to prove knowledge of the spending key authorizing spending of an input note.
    #[serde(rename = "spendAuthSig")]
    pub spend_auth_sig: String,
}

impl ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVShieldedSpend {
    pub fn new(anchor: String, cv: String, nullifier: String, proof: String, rk: String, spend_auth_sig: String) -> ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVShieldedSpend {
        ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVShieldedSpend {
            anchor,
            cv,
            nullifier,
            proof,
            rk,
            spend_auth_sig,
        }
    }
}

