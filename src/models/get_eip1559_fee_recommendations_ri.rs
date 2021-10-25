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
pub struct GetEip1559FeeRecommendationsRi {
    #[serde(rename = "baseFeePerGas")]
    pub base_fee_per_gas: Box<crate::models::GetEip1559FeeRecommendationsRiBaseFeePerGas>,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Box<crate::models::GetEip1559FeeRecommendationsRiMaxFeePerGas>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Box<crate::models::GetEip1559FeeRecommendationsRiMaxPriorityFeePerGas>,
}

impl GetEip1559FeeRecommendationsRi {
    pub fn new(base_fee_per_gas: crate::models::GetEip1559FeeRecommendationsRiBaseFeePerGas, max_fee_per_gas: crate::models::GetEip1559FeeRecommendationsRiMaxFeePerGas, max_priority_fee_per_gas: crate::models::GetEip1559FeeRecommendationsRiMaxPriorityFeePerGas) -> GetEip1559FeeRecommendationsRi {
        GetEip1559FeeRecommendationsRi {
            base_fee_per_gas: Box::new(base_fee_per_gas),
            max_fee_per_gas: Box::new(max_fee_per_gas),
            max_priority_fee_per_gas: Box::new(max_priority_fee_per_gas),
        }
    }
}


