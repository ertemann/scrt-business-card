use cosmwasm_std::Addr;
use schemars::JsonSchema;

use serde::{Deserialize, Serialize};

use crate::state::Card;
// use crate::state::{ScannedViewingKey};
// use secret_toolkit::permit::Permit;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InitMsg {
    pub entropy: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Create { card: Card, index: u8 },
    Burn { index: u8 },
    GenerateViewingKey { index: u8 },
    // ScanViewingKey {
    //     wallet: Addr,
    //     viewing_key: String,
    //     index: u8,
    // },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ViewingKeyResponse {
    pub viewing_key: String,
}

// We define a custom struct for each query response
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub struct ViewingKeysResponse {
//     pub viewing_keys: Vec<ScannedViewingKey>,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCard {
        wallet: Addr,
        viewing_key: String,
        index: u8,
    },
    // GetAllViewingKeys {
    //     viewing_keys_permit: Permit,
    // },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CardResponse {
    pub card: Card,
}
