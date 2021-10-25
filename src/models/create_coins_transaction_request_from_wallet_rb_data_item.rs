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
pub struct CreateCoinsTransactionRequestFromWalletRbDataItem {
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security).
    #[serde(rename = "callbackSecretKey", skip_serializing_if = "Option::is_none")]
    pub callback_secret_key: Option<String>,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs.
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\".
    #[serde(rename = "feePriority")]
    pub fee_priority: FeePriority,
    /// Represents an optional note to add a free text in, explaining or providing additional detail on the transaction request.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Refers to a model of a UTXO spending strategy, where customers can choose how to spend their transaction outputs from multiple Bitcoin addresses. Two options available - \"minimize-dust\" (select lower amounts from multiple addresses) or \"optimize-size\" (select higher amounts from less addresses).
    #[serde(rename = "prepareStrategy", skip_serializing_if = "Option::is_none")]
    pub prepare_strategy: Option<PrepareStrategy>,
    /// Defines the destination of the transaction, whether it is incoming or outgoing.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::CreateCoinsTransactionRequestFromWalletRbDataItemRecipients>,
}

impl CreateCoinsTransactionRequestFromWalletRbDataItem {
    pub fn new(fee_priority: FeePriority, recipients: Vec<crate::models::CreateCoinsTransactionRequestFromWalletRbDataItemRecipients>) -> CreateCoinsTransactionRequestFromWalletRbDataItem {
        CreateCoinsTransactionRequestFromWalletRbDataItem {
            callback_secret_key: None,
            callback_url: None,
            fee_priority,
            note: None,
            prepare_strategy: None,
            recipients,
        }
    }
}

/// Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeePriority {
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "fast")]
    Fast,
}
/// Refers to a model of a UTXO spending strategy, where customers can choose how to spend their transaction outputs from multiple Bitcoin addresses. Two options available - \"minimize-dust\" (select lower amounts from multiple addresses) or \"optimize-size\" (select higher amounts from less addresses).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PrepareStrategy {
    #[serde(rename = "minimize-dust")]
    MinimizeDust,
    #[serde(rename = "optimize-size")]
    OptimizeSize,
}

