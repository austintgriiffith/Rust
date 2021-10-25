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
pub struct ListAllUnconfirmedTransactionsRi {
    /// Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::ListUnconfirmedTransactionsByAddressRiRecipients>,
    /// Represents a list of sender addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "senders")]
    pub senders: Vec<crate::models::ListUnconfirmedTransactionsByAddressRiSenders>,
    /// Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "blockchainSpecific")]
    pub blockchain_specific: Box<crate::models::ListAllUnconfirmedTransactionsRibs>,
}

impl ListAllUnconfirmedTransactionsRi {
    pub fn new(recipients: Vec<crate::models::ListUnconfirmedTransactionsByAddressRiRecipients>, senders: Vec<crate::models::ListUnconfirmedTransactionsByAddressRiSenders>, timestamp: i32, transaction_id: String, blockchain_specific: crate::models::ListAllUnconfirmedTransactionsRibs) -> ListAllUnconfirmedTransactionsRi {
        ListAllUnconfirmedTransactionsRi {
            recipients,
            senders,
            timestamp,
            transaction_id,
            blockchain_specific: Box::new(blockchain_specific),
        }
    }
}


