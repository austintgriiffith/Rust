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
pub struct ListZilliqaTransactionsByBlockHashRi {
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
    /// Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\".
    #[serde(rename = "minedInBlockHeight")]
    pub mined_in_block_height: i32,
    /// Represents a random value that can be adjusted to satisfy the Proof of Work.
    #[serde(rename = "nonce")]
    pub nonce: i32,
    /// Defines an object array of the transaction recipients.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::ListZilliqaTransactionsByAddressRiRecipients>,
    /// Represents an object of addresses that provide the funds.
    #[serde(rename = "senders")]
    pub senders: Vec<crate::models::ListZilliqaTransactionsByAddressRiSenders>,
    /// Defines the exact date/time when this block was mined in Unix Timestamp.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the hash of the transaction, which is its unique identifier.
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    /// Defines the numeric representation of the transaction index.
    #[serde(rename = "transactionIndex")]
    pub transaction_index: i32,
    /// Defines the status of the transaction, whether it is e.g. pending or complete.
    #[serde(rename = "transactionStatus")]
    pub transaction_status: String,
}

impl ListZilliqaTransactionsByBlockHashRi {
    pub fn new(fee: crate::models::GetZilliqaTransactionDetailsByTransactionIdriFee, gas_limit: i32, gas_price: i32, gas_used: i32, mined_in_block_height: i32, nonce: i32, recipients: Vec<crate::models::ListZilliqaTransactionsByAddressRiRecipients>, senders: Vec<crate::models::ListZilliqaTransactionsByAddressRiSenders>, timestamp: i32, transaction_hash: String, transaction_index: i32, transaction_status: String) -> ListZilliqaTransactionsByBlockHashRi {
        ListZilliqaTransactionsByBlockHashRi {
            fee: Box::new(fee),
            gas_limit,
            gas_price,
            gas_used,
            mined_in_block_height,
            nonce,
            recipients,
            senders,
            timestamp,
            transaction_hash,
            transaction_index,
            transaction_status,
        }
    }
}


