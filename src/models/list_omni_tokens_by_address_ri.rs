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
pub struct ListOmniTokensByAddressRi {
    /// Defines the balance of the Omni tokens to send in the address.
    #[serde(rename = "balance")]
    pub balance: String,
    /// Defines the amount frozen by the issuer (applies to managed properties only).
    #[serde(rename = "frozen")]
    pub frozen: String,
    /// Defines the name of the Omni tokens to send.
    #[serde(rename = "name")]
    pub name: String,
    /// Represents the identifier of the tokens to send.
    #[serde(rename = "propertyId")]
    pub property_id: i32,
    /// Represents the amount reserved by sell offers and accepts.
    #[serde(rename = "reserved")]
    pub reserved: String,
}

impl ListOmniTokensByAddressRi {
    pub fn new(balance: String, frozen: String, name: String, property_id: i32, reserved: String) -> ListOmniTokensByAddressRi {
        ListOmniTokensByAddressRi {
            balance,
            frozen,
            name,
            property_id,
            reserved,
        }
    }
}


