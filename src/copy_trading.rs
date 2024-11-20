use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::macros::my_no_sql_entity;
use service_sdk::my_no_sql_sdk;

#[my_no_sql_entity("copy-trading-subscriptions")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MyNoSqlSubscription{
    pub id: String,
    pub provider_id: String,
    pub trader_id: String,
    pub account_id: String,
    pub enabled: bool,
    pub pending_copy: bool,
    pub sl_tp_copy: bool,
    pub copy_multiplier: f64,
}

impl MyNoSqlSubscription {
    pub fn get_pk() -> String{
        "cps".to_string()
    }
}

#[my_no_sql_entity("copy-trading-providers")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MyNoSqlProvider{
    pub id: String,
    pub trader_id: String,
    pub account_id: String,
    pub enabled_at: DateTime<Utc>,
    pub enabled: bool,
    pub provider_commission: Option<f64>
}

impl MyNoSqlProvider {
    pub fn get_pk() -> String{
        "cpp".to_string()
    }
}