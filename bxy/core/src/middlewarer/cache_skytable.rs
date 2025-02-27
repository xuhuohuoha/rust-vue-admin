use core::time::Duration;
use std::{collections::BTreeMap, sync::Arc, time::Instant};

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use once_cell::sync::Lazy;
use skytable::{pool::ConnectionMgrTls, ConnectionAsync};
use tokio::sync::{Mutex, OnceCell};

use crate::{
    api::response::res::ResJsonString,
    config::CONFIG,
    model::app_structs::{ApiInfo, RequestContext, UserInfoContext},
    serve::utils::ApiUtils::ALL_APIS,
};

static SKY: OnceCell<ConnectionAsync> = OnceCell::const_new();

async fn set_sky() -> ConnectionAsync {
    let notls_manager = ConnectionMgrTls::new(&CONFIG.skytable.server, CONFIG.skytable.port);
    let notls_pool = ConnectionAsync::new
        .max_size(10)
        .build(notls_manager)
        .await
        .expect("skytable connect error please check it");
    //  第一次连接时，也就是程序启动时，清空数据
    notls_pool
        .get()
        .await
        .expect("skytable connect error please check it")
        .flushdb()
        .await
        .unwrap();

    notls_pool
}

//  定义一个skytable 连接
pub async fn get_sky_table() -> &'static ConnectionAsync {
    SKY.get_or_init(set_sky).await as _
}

// 程序中定义一个全局Map
// 用于存储已经缓存的数据，如果数据有更新,就清除该数据中对应的键值,
// 下次请求重新请求数据
pub static INDEX_MAP: Lazy<Arc<Mutex<BTreeMap<String, Instant>>>> = Lazy::new(|| {
    let data: BTreeMap<String, Instant> = BTreeMap::new();
    tokio::spawn(async { self::init().await });
    Arc::new(Mutex::new(data))
});
pub async fn init() {
    tracing::info!("cache data init");

    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        init_loop().await;
    }
}

async fn init_loop() {
    let d = CONFIG.server.cache_time * 1000;
    let mut index = INDEX_MAP.lock().await;
    let mut data_keys: Vec<String> = Vec::new();
    for (k, v) in index.clone().iter() {
        if Instant::now().duration_since(*v).as_millis() as u64 > d {
            let key = k.split('★').collect::<Vec<&str>>();
            data_keys.push(key[1].to_string());
            // 移除缓存索引
            index.remove(k);
        }
    }
    drop(index);
    if !data_keys.is_empty() {
        // 移除缓存数据
        remove_cache_data(data_keys).await;
    }
}
//  添加索引
async fn add_index_map(api_key: &str, data_key: &str) {
    let mut index = INDEX_MAP.lock().await;
    let key = get_key(api_key, data_key);
    index.insert(key, Instant::now());
    drop(index);
}
//  删除索引
async fn remove_index_map(api_key: Option<String>) {
    let mut index = INDEX_MAP.lock().await;
    let mut data_keys: Vec<String> = Vec::new();
    if let Some(api_key) = api_key {
        for (k, _) in index.clone().into_iter() {
            if k.starts_with(&api_key) {
                index.remove(&k);
                let key = k.split('★').collect::<Vec<&str>>();
                data_keys.push(key[1].to_string())
            }
        }
    }
    remove_cache_data(data_keys).await;
    drop(index);
}

//  获取索引是否存在
async fn get_index_map(api_key: &str, data_key: &str) -> bool {
    let key = get_key(api_key, data_key);
    let index = INDEX_MAP.lock().await;
    let res = index.get(&key).is_some();
    drop(index);
    res
}

//  获取key
fn get_key(api_key: &str, data_key: &str) -> String {
    format!("{}★{}", api_key, &data_key)
}
//  添加数据
pub async fn add_cache_data(ori_uri: &str, api_key: &str, data_key: &str, data: String) {
    let con = get_sky_table().await;

    add_index_map(api_key, data_key).await;

    match con.get().await.unwrap().get::<String>(data_key).await {
        Ok(_) => match con.get().await.unwrap().update(data_key, data).await {
            Ok(_) => tracing::info!(
                "update cache data OK,api_key: {}, data_key: {},api:{}",
                api_key,
                data_key,
                ori_uri
            ),
            Err(_) => tracing::info!(
                "update cache data error,api_key: {}, data_key: {},api:{}",
                api_key,
                data_key,
                ori_uri
            ),
        },
        Err(_) => match con.get().await.unwrap().set(data_key, data).await {
            Ok(_) => tracing::info!(
                "add cache data OK,api_key: {}, data_key: {},api:{}",
                api_key,
                data_key,
                ori_uri
            ),
            Err(_) => tracing::info!(
                "add cache data error,api_key: {}, data_key: {},api:{}",
                api_key,
                data_key,
                ori_uri
            ),
        },
    };
}

//  获取数据
pub async fn get_cache_data(api_key: &str, data_key: &str) -> Option<String> {
    let con = get_sky_table().await;

    match get_index_map(api_key, data_key).await {
        false => None,
        true => {
            let data: Option<String> = match con.get().await.unwrap().get::<String>(data_key).await
            {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            tracing::info!(
                "get cache data success,api_key: {}, data_key: {}",
                api_key,
                data_key
            );
            data
        }
    }
}

//  移除数据
pub async fn remove_cache_data(data_keys: Vec<String>) {
    let con = get_sky_table().await;
    match con.get().await.unwrap().del(&data_keys).await {
        Ok(v) => tracing::info!(
            "remove cache data success,data_keys: {:?},total:{}",
            &data_keys,
            v
        ),
        Err(e) => tracing::info!(
            "remove cache data failed,data_keys: {:?},error:{}",
            &data_keys,
            e
        ),
    }
}
//  缓存中间件
pub async fn cache_fn_mid(req: Request, next: Next) -> Result<Response, StatusCode> {
    let apis = ALL_APIS.lock().await;
    let ctx = req
        .extensions()
        .get::<RequestContext>()
        .expect("ReqCtx not found")
        .clone();
    let ctx_user = match req.extensions().get::<UserInfoContext>() {
        Some(v) => v.to_owned(),
        None => return Ok(next.run(req).await),
    };
    let api_info = match apis.get(&ctx.path) {
        Some(x) => x.clone(),
        None => ApiInfo {
            name: "".to_string(),
            data_cache_method: "0".to_string(),
            log_method: "0".to_string(),
            api: None,
        },
    };
    // 释放锁
    drop(apis);
    let token_id = ctx_user.token;

    if ctx.method.as_str() != "GET" {
        let res_end = next.run(req).await;
        return match res_end.status() {
            StatusCode::OK => {
                tokio::spawn(async move {
                    remove_index_map(api_info.api.clone()).await;
                });
                Ok(res_end)
            }
            _ => Ok(res_end),
        };
    }
    let data_key = match api_info.data_cache_method.clone().as_str() {
        "1" => format!("{}_{}_{}", &ctx.ori_uri, &ctx.method, &token_id),
        _ => format!("{}_{}", &ctx.ori_uri, &ctx.method),
    };
    // 开始请求数据
    match api_info.data_cache_method.as_str() {
        "0" => {
            let res_end = next.run(req).await;
            Ok(res_end)
        }
        _ => match get_cache_data(&ctx.path, &data_key).await {
            Some(v) => Ok(v.into_response()),

            None => {
                let res_end = next.run(req).await;
                match res_end.status() {
                    StatusCode::OK => {
                        let res_ctx = match res_end.extensions().get::<ResJsonString>() {
                            Some(x) => x.0.clone(),
                            None => "".to_string(),
                        };

                        tokio::spawn(async move {
                            // 缓存数据
                            add_cache_data(&ctx.ori_uri, &ctx.path, &data_key, res_ctx).await;
                        });

                        Ok(res_end)
                    }
                    _ => Ok(res_end),
                }
            }
        },
    }
}
