/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// AddressTokensTransactionConfirmedEachConfirmationErc20 : ERC-20



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressTokensTransactionConfirmedEachConfirmationErc20 {
    /// Specifies the name of the token.
    #[serde(rename = "name")]
    pub name: String,
    /// Specifies an identifier of the token, where up to five alphanumeric characters can be used for it.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Defines how many decimals can be used to break the token.
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<String>,
    /// Defines the amount of tokens sent with the confirmed transaction.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the address of the contract.
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}

impl AddressTokensTransactionConfirmedEachConfirmationErc20 {
    /// ERC-20
    pub fn new(name: String, symbol: String, amount: String, contract_address: String) -> AddressTokensTransactionConfirmedEachConfirmationErc20 {
        AddressTokensTransactionConfirmedEachConfirmationErc20 {
            name,
            symbol,
            decimals: None,
            amount,
            contract_address,
        }
    }
}


