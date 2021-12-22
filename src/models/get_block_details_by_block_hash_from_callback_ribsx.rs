/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetBlockDetailsByBlockHashFromCallbackRibsx : XRP



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBlockDetailsByBlockHashFromCallbackRibsx {
    #[serde(rename = "totalCoins")]
    pub total_coins: Box<crate::models::GetLatestMinedXrpRippleBlockRiTotalCoins>,
    #[serde(rename = "totalFees")]
    pub total_fees: Box<crate::models::GetLatestMinedXrpRippleBlockRiTotalFees>,
}

impl GetBlockDetailsByBlockHashFromCallbackRibsx {
    /// XRP
    pub fn new(total_coins: crate::models::GetLatestMinedXrpRippleBlockRiTotalCoins, total_fees: crate::models::GetLatestMinedXrpRippleBlockRiTotalFees) -> GetBlockDetailsByBlockHashFromCallbackRibsx {
        GetBlockDetailsByBlockHashFromCallbackRibsx {
            total_coins: Box::new(total_coins),
            total_fees: Box::new(total_fees),
        }
    }
}

