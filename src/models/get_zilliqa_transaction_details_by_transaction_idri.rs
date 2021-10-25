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
pub struct GetZilliqaTransactionDetailsByTransactionIdri {
    #[serde(rename = "fee")]
    pub fee: Box<crate::models::GetZilliqaTransactionDetailsByTransactionIdriFee>,
    /// Represents the maximum amount of gas allowed in the block in order to determine how many transactions it can fit.
    #[serde(rename = "gasLimit")]
    pub gas_limit: i32,
    /// Defines the price of the gas.
    #[serde(rename = "gasPrice")]
    pub gas_price: i32,
    /// Defines how much of the gas for the block has been used.
    #[serde(rename = "gasUsed")]
    pub gas_used: i32,
    /// Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "minedInBlockHash")]
    pub mined_in_block_hash: String,
    /// Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\".
    #[serde(rename = "minedInBlockHeight")]
    pub mined_in_block_height: i32,
    /// Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address.
    #[serde(rename = "nonce")]
    pub nonce: i32,
    /// Represents an object of addresses that receive the transactions.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::GetZilliqaTransactionDetailsByTransactionIdriRecipients>,
    /// Represents an object of addresses that provide the funds.
    #[serde(rename = "senders")]
    pub senders: Vec<crate::models::GetZilliqaTransactionDetailsByTransactionIdriSenders>,
    /// Defines the exact date/time when this block was mined in Unix Timestamp.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Defines the numeric representation of the transaction index.
    #[serde(rename = "transactionIndex")]
    pub transaction_index: i32,
    /// Defines the status of the transaction, whether it is e.g. pending or complete.
    #[serde(rename = "transactionStatus")]
    pub transaction_status: String,
}

impl GetZilliqaTransactionDetailsByTransactionIdri {
    pub fn new(fee: crate::models::GetZilliqaTransactionDetailsByTransactionIdriFee, gas_limit: i32, gas_price: i32, gas_used: i32, mined_in_block_hash: String, mined_in_block_height: i32, nonce: i32, recipients: Vec<crate::models::GetZilliqaTransactionDetailsByTransactionIdriRecipients>, senders: Vec<crate::models::GetZilliqaTransactionDetailsByTransactionIdriSenders>, timestamp: i32, transaction_index: i32, transaction_status: String) -> GetZilliqaTransactionDetailsByTransactionIdri {
        GetZilliqaTransactionDetailsByTransactionIdri {
            fee: Box::new(fee),
            gas_limit,
            gas_price,
            gas_used,
            mined_in_block_hash,
            mined_in_block_height,
            nonce,
            recipients,
            senders,
            timestamp,
            transaction_index,
            transaction_status,
        }
    }
}


