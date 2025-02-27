//! 操作日志实体
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "bxy_oper_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    pub time_id: i64,
    pub title: String,
    /// 业务类型
    pub business_type: String,
    /// 方法
    pub method: String,
    /// 请求方式
    pub request_method: String,
    /// 操作类型
    pub operator_type: String,
    /// 操作人
    pub oper_name: String,
    /// 操作接口
    pub oper_url: String,
    /// 操作ip
    pub oper_ip: String,
    /// 操作地点
    pub oper_location: String,
    /// 操作参数
    pub oper_param: String,
    /// 路径参数
    pub path_param: String,
    /// 操作结果
    pub json_result: String,
    /// 状态
    pub status: String,
    /// 错误信息
    pub error_msg: String,
    /// 耗时
    pub duration: i64,
    /// 操作时间
    pub oper_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
