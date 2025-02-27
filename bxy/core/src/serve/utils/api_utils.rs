//! Service Api Toolkit
//!

use byz_entity::core::{sys_user, UserEntity};
use once_cell::sync::Lazy;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    db::{db_conn, DB},
    model::app_structs::ApiInfo,
    serve::{srv_menu, srv_user_api},
};

/// 将所有 Api 加载到内存中
pub static ALL_APIS: Lazy<Arc<Mutex<HashMap<String, ApiInfo>>>> = Lazy::new(|| {
    let apis: HashMap<String, ApiInfo> = HashMap::new();
    Arc::new(Mutex::new(apis))
});

/// 初始化所有 Api
pub async fn init_all_api() {
    init_api().await;
}

/// 重置所有 Api
pub async fn reset_api() {
    let mut apis = ALL_APIS.lock().await;
    apis.clear();
    drop(apis);
    init_api().await;
}

/// Api 初始化方法
async fn init_api() {
    let db = DB.get_or_init(db_conn).await;
    let res = srv_menu::find_api(db, false, false, false).await;
    match res {
        Ok(menus) => {
            for menu in menus {
                self::add_api(
                    &menu.guid,
                    &menu.api,
                    &menu.alias,
                    &menu.data_cache_method,
                    &menu.log_method,
                )
                .await;
            }
            let apis = ALL_APIS.lock().await;
            tracing::info!("初始化时获取路由API成功.");
            drop(apis);
        }
        Err(e) => {
            tracing::info!("初始化时获取路由API失败:{:#?}", e)
        }
    }
}

pub async fn add_api(
    api_id: &str,
    api: &str,
    menu_name: &str,
    data_cache_method: &str,
    log_method: &str,
) {
    let api_info = ApiInfo {
        api_id: api_id.to_string(),
        name: menu_name.to_string(),
        api: Some(api.to_string()),
        data_cache_method: data_cache_method.to_string(),
        log_method: log_method.to_string(),
    };
    let mut apis = ALL_APIS.lock().await;
    apis.entry(api.to_string())
        .and_modify(|x| {
            *x = api_info.clone();
        })
        .or_insert(api_info);
    drop(apis)
}

/// 移除 Api
pub async fn remove_api(api: &str) {
    let mut apis = ALL_APIS.lock().await;
    apis.remove(api);
    drop(apis)
}

/// Api 是否已存在
pub async fn is_api_in(api: &str) -> bool {
    let apis = ALL_APIS.lock().await;
    let res = apis.get(api).is_some();
    drop(apis);
    res
}

/// 校验 Api 权限
pub async fn check_api_permission(u_id: &str, mcode: &str, api: &str, method: &str) -> bool {
    let db = DB.get_or_init(db_conn).await;
    match UserEntity::find()
        .filter(sys_user::Column::UId.eq(u_id))
        .one(db)
        .await
    {
        Ok(v) => match v {
            Some(user) => user.id,
            None => {
                tracing::info!("未查找到当前用户:{:?}", &u_id);
                return false;
            }
        },
        Err(e) => {
            tracing::info!("查找用户出现错误:{:#?}", e);
            return false;
        }
    };
    match srv_user_api::check_api_permission(db, u_id, mcode, api, method).await {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    }
}
