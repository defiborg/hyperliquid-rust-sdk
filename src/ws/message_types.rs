use crate::ws::sub_structs::*;
use serde::Deserialize;

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct Trades {
    pub data: Vec<Trade>,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct L2Book {
    pub data: L2BookData,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct AllMids {
    pub data: AllMidsData,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct User {
    pub data: UserData,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct UserFills {
    pub data: UserFillsData,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct Candle {
    pub data: CandleData,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct OrderUpdates {
    pub data: Vec<OrderUpdate>,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct UserFundings {
    pub data: UserFundingsData,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct UserNonFundingLedgerUpdates {
    pub data: UserNonFundingLedgerUpdatesData,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct Notification {
    pub data: NotificationData,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct WebData2 {
    pub data: WebData2Data,
}

#[derive(Deserialize, serde::Serialize, Debug, Clone)]
pub struct ActiveAssetCtx {
    pub data: ActiveAssetCtxData,
}
