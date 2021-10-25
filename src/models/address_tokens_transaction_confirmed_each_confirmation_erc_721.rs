/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// AddressTokensTransactionConfirmedEachConfirmationErc721 : ERC-721



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressTokensTransactionConfirmedEachConfirmationErc721 {
    /// Specifies the name of the token.
    #[serde(rename = "name")]
    pub name: String,
    /// Specifies an identifier of the token, where up to five alphanumeric characters can be used for it.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Specifies the ID of the token.
    #[serde(rename = "tokenId")]
    pub token_id: String,
    /// Specifies the address of the contract.
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}

impl AddressTokensTransactionConfirmedEachConfirmationErc721 {
    /// ERC-721
    pub fn new(name: String, symbol: String, token_id: String, contract_address: String) -> AddressTokensTransactionConfirmedEachConfirmationErc721 {
        AddressTokensTransactionConfirmedEachConfirmationErc721 {
            name,
            symbol,
            token_id,
            contract_address,
        }
    }
}


