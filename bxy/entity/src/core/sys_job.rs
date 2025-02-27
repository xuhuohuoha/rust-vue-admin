//! 定时作业实体
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "bxy_job")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 业务主键
    pub guid: String,
    /// 创建人
    pub create_by: String,
    /// 最近一次修改人
    pub update_by: Option<String>,
    /// 逻辑删除人
    pub delete_by: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
    /// 修改时间
    pub updated_at: Option<DateTime>,
    /// 删除时间
    pub deleted_at: Option<DateTime>,
    /// 版本号
    pub version: u32,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 任务次数
    pub task_count: i64,
    /// 已执行次数
    pub run_count: i64,
    /// 任务名称
    pub job_name: String,
    /// 任务参数
    pub job_params: Option<String>,
    /// 任务分组
    pub job_group: String,
    /// 调用目标
    pub invoke_target: String,
    /// 条件表达式
    pub cron_expression: String,
    pub misfire_policy: String,
    pub concurrent: Option<String>,
    /// 最后一次执行时间
    pub last_time: Option<DateTime>,
    /// 下一次执行时间
    pub next_time: Option<DateTime>,
    /// 执行结束时间
    pub end_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
