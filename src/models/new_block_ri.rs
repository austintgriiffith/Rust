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
pub struct NewBlockRi {
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security).
    #[serde(rename = "callbackSecretKey")]
    pub callback_secret_key: String,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs.
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// Defines the specific time/date when the subscription was created in Unix Timestamp.
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: i32,
    /// Defines whether the subscription is active or not. Set as boolean.
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Represents a unique ID used to reference the specific callback subscription.
    #[serde(rename = "referenceId")]
    pub reference_id: String,
}

impl NewBlockRi {
    pub fn new(callback_secret_key: String, callback_url: String, created_timestamp: i32, is_active: bool, reference_id: String) -> NewBlockRi {
        NewBlockRi {
            callback_secret_key,
            callback_url,
            created_timestamp,
            is_active,
            reference_id,
        }
    }
}


