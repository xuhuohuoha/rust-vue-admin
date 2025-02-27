use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct OperLogSearchReq {
    pub oper_name: Option<String>,
}
