/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVJoinSplit {
    /// Defines a Merkle tree root of a note commitment tree which uniquely identifies a note commitment tree state given the assumed security properties of the Merkle tree’s hash function.
    #[serde(rename = "anchor")]
    pub anchor: String,
    #[serde(rename = "cipherTexts")]
    pub cipher_texts: Vec<String>,
    #[serde(rename = "commitments")]
    pub commitments: Vec<String>,
    #[serde(rename = "macs")]
    pub macs: Vec<String>,
    #[serde(rename = "nullifiers")]
    pub nullifiers: Vec<String>,
    /// Defines the one time public key.
    #[serde(rename = "oneTimePubKey")]
    pub one_time_pub_key: String,
    /// Defines the proof.
    #[serde(rename = "proof")]
    pub proof: String,
    /// Represents a 256-bit seed that must be chosen independently at random for each JoinSplit description.
    #[serde(rename = "randomSeed")]
    pub random_seed: String,
    /// Defines the value that the joinSplit transfer will insert into the transparent transaction value pool.
    #[serde(rename = "vPubNew")]
    pub v_pub_new: String,
    /// Defines the value that the joinSplit transfer will remove from the transparent transaction value pool.
    #[serde(rename = "vPubOld")]
    pub v_pub_old: String,
}

impl ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVJoinSplit {
    pub fn new(anchor: String, cipher_texts: Vec<String>, commitments: Vec<String>, macs: Vec<String>, nullifiers: Vec<String>, one_time_pub_key: String, proof: String, random_seed: String, v_pub_new: String, v_pub_old: String) -> ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVJoinSplit {
        ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVJoinSplit {
            anchor,
            cipher_texts,
            commitments,
            macs,
            nullifiers,
            one_time_pub_key,
            proof,
            random_seed,
            v_pub_new,
            v_pub_old,
        }
    }
}


