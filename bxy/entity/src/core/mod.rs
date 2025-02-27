//! Blunka BPM Core Entity
//!
//! # 描述
//! 伯伦卡业务流程平台核心功能数据库实体

pub mod sys_adtion;
pub mod sys_adtion_ex;
pub mod sys_app;
pub mod sys_app_auth;
pub mod sys_col_auth;
pub mod sys_datasource;
pub mod sys_dict;
pub mod sys_dict_data;
pub mod sys_dql;
pub mod sys_duty;
pub mod sys_form_auth;
pub mod sys_job;
pub mod sys_job_log;
pub mod sys_login_log;
pub mod sys_master_detail;
pub mod sys_menu;
pub mod sys_online;
pub mod sys_oper_log;
pub mod sys_org;
pub mod sys_post;
pub mod sys_role;
pub mod sys_row_auth;
pub mod sys_tree;
pub mod sys_update_log;
pub mod sys_user;
pub mod sys_user_api;
pub mod sys_user_auth;
pub mod sys_user_duty;
pub mod sys_user_org;
pub mod sys_user_post;
pub mod sys_user_role;

pub use crate::core::{
    sys_adtion::Entity as AdtionEntity, sys_adtion_ex::Entity as AdtionExEntity,
    sys_app::Entity as AppEntity, sys_app_auth::Entity as AppAuthEntity,
    sys_col_auth::Entity as ColAuthEntity, sys_datasource::Entity as DatasourceEntity,
    sys_dict::Entity as DictEntity, sys_dict_data::Entity as DictDataEntity,
    sys_dql::Entity as DqlEntity, sys_duty::Entity as DutyEntity,
    sys_form_auth::Entity as FormAuthEntity, sys_job::Entity as JobEntity,
    sys_job_log::Entity as JobLogEntity, sys_login_log::Entity as LoginLogEntity,
    sys_master_detail::Entity as MdEntity, sys_menu::Entity as MenuEntity,
    sys_online::Entity as OnlineEntity, sys_oper_log::Entity as OperLogEntity,
    sys_org::Entity as OrgEntity, sys_post::Entity as PostEntity, sys_role::Entity as RoleEntity,
    sys_row_auth::Entity as RowAuthEntity, sys_tree::Entity as TreeEntity,
    sys_update_log::Entity as UpdateLogEntity, sys_user::Entity as UserEntity,
    sys_user_api::Entity as UserApiEntity, sys_user_auth::Entity as UserAuthEntity,
    sys_user_duty::Entity as UserDutyEntity, sys_user_org::Entity as UserOrgEntity,
    sys_user_post::Entity as UserPostEntity, sys_user_role::Entity as UserRoleEntity,
};

pub use crate::core::{
    sys_adtion::Model as AdtionModel, sys_adtion_ex::Model as AdtionExModel,
    sys_app::Model as AppModel, sys_app_auth::Model as AppAuthModel,
    sys_col_auth::Model as ColAuthModel, sys_datasource::Model as DatasourceModel,
    sys_dict::Model as DictModel, sys_dict_data::Model as DictDataModel,
    sys_dql::Model as DqlModel, sys_duty::Model as DutyModel,
    sys_form_auth::Model as FormAuthModel, sys_job::Model as JobModel,
    sys_job_log::Model as JobLogModel, sys_login_log::Model as LoginLogModel,
    sys_master_detail::Model as MdModel, sys_menu::Model as MenuModel,
    sys_online::Model as OnlineModel, sys_oper_log::Model as OperLogModel,
    sys_org::Model as OrgModel, sys_post::Model as PostModel, sys_role::Model as RoleModel,
    sys_row_auth::Model as RowAuthModel, sys_tree::Model as TreeModel,
    sys_update_log::Model as UpdateLogModel, sys_user::Model as UserModel,
    sys_user_api::Model as UserApiModel, sys_user_auth::Model as UserAuthModel,
    sys_user_duty::Model as UserDutyModel, sys_user_org::Model as UserOrgModel,
    sys_user_post::Model as UserPostModel, sys_user_role::Model as UserRoleModel,
};
