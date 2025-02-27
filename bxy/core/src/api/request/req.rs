//! 前端通用请求体结构
//!

use sea_orm::Value;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
/// 表单请求体结构
pub struct FormReq {
    /// 菜單 GUID
    pub mcode: String,
    /// 表单
    pub frm: JsonValue,
    /// 子表单
    pub frms: Vec<FormReq>,
}

#[derive(Deserialize, ToSchema)]
/// 查询请求结构体
pub struct SearchReq {
    /// 动态脚本标识
    pub sign: String,
    /// 参数集合
    #[serde(default)]
    pub params: Vec<JsonValue>,
}
impl SearchReq {
    pub fn to_value_vec(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        for param in self.params.clone() {
            match param {
                JsonValue::Null => values.push(Value::String(Some(Box::new("".to_string())))),
                JsonValue::Bool(b) => values.push(Value::Bool(Some(b))),
                JsonValue::Number(n) => {
                    if n.is_i64() {
                        values.push(Value::BigInt(Some(n.as_i64().unwrap())));
                    } else {
                        values.push(Value::Double(Some(n.as_f64().unwrap())));
                    }
                }
                JsonValue::String(s) => values.push(Value::String(Some(Box::new(s)))),
                JsonValue::Array(_arr) => {
                    values.extend(self.to_value_vec().into_iter());
                }
                JsonValue::Object(_) => values.push(Value::String(Some(Box::new("".to_string())))),
            }
        }
        values
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
/// 删除请求体结构
pub struct DeleteReq {
    /// ID 集合
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
/// 密码加密请求体结构
pub struct EncryptPasswordReq {
    /// 待加密密码
    pub password: String,
    /// 盐
    pub salt: String,
}
