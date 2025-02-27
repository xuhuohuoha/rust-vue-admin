//!  日志中间件
//!

use core::time::Duration;
use std::time::Instant;

use anyhow::Result;
use axum::{extract::Request, middleware::Next, response::IntoResponse};
use chrono::Local;
use hyper::StatusCode;

use crate::{
    api::response::res::ResJsonString,
    config::CONFIG,
    model::{
        app_structs::{RequestContext, UserInfoContext},
        dict_structs::OperResult,
    },
    serve::utils::ApiUtils::ALL_APIS,
};

use crate::serve::srv_oper_log;

/// 操作日志中间件
pub async fn oper_log_fn_mid(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 查询ctx
    let req_ctx = match req.extensions().get::<RequestContext>() {
        Some(x) => x.clone(),
        None => return Ok(next.run(req).await),
    };

    let ctx_user = match req.extensions().get::<UserInfoContext>() {
        Some(x) => x.clone(),
        None => return Ok(next.run(req).await),
    };

    let now = Instant::now();
    let res_end = next.run(req).await;
    let duration = now.elapsed();
    let res_string = match res_end.extensions().get::<ResJsonString>() {
        Some(x) => x.0.clone(),
        None => "".to_string(),
    };
    oper_log_add(
        req_ctx,
        ctx_user,
        res_string,
        OperResult::Success.to_string(),
        "".to_string(),
        duration,
    )
    .await;
    Ok(res_end)
}

/// 新增操作日志
pub async fn oper_log_add(
    ctx: RequestContext,
    ctx_user: UserInfoContext,
    res: String,
    status: String,
    err_msg: String,
    duration: Duration,
) {
    tokio::spawn(async move {
        match oper_log_add_fn(ctx, ctx_user, res, status, err_msg, duration).await {
            Ok(_) => {}
            Err(e) => {
                tracing::info!("日志添加失败：{}", e.to_string());
            }
        };
    });
}

/// 新增操作日志
///
/// # 描述
/// - 文件日志
/// - 数据库日志
/// - 文件日志 + 数据库日志
async fn oper_log_add_fn(
    ctx: RequestContext,
    ctx_user: UserInfoContext,
    res: String,
    status: String,
    err_msg: String,
    duration: Duration,
) -> Result<()> {
    if !CONFIG.log.enable_oper_log {
        return Ok(());
    }
    let apis = ALL_APIS.lock().await;
    let (api_name, is_log) = match apis.get(&ctx.path) {
        Some(x) => (x.name.clone(), x.log_method.clone()),
        None => ("".to_string(), "0".to_string()),
    };
    drop(apis);
    let now = Local::now().naive_local();
    // 打印日志
    let req_data = ctx.clone();
    let res_data = res.clone();
    let err_msg_data = err_msg.clone();
    let duration_data = duration;
    match is_log.as_str() {
        // 文件日志
        "1" => {
            tokio::spawn(async move {
                file_log(req_data, now, duration_data, res_data, err_msg_data);
            });
        }
        // 数据库日志
        "2" => {
            tokio::spawn(async move {
                match srv_oper_log::db_log(
                    duration_data,
                    ctx,
                    ctx_user,
                    now,
                    api_name,
                    res,
                    status,
                    err_msg,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::info!("日志添加失败：{}", e.to_string());
                    }
                };
            });
        }
        // 文件日志 + 数据库日志
        "3" => {
            tokio::spawn(async move {
                file_log(req_data, now, duration_data, res_data, err_msg_data);
                match srv_oper_log::db_log(
                    duration_data,
                    ctx,
                    ctx_user,
                    now,
                    api_name,
                    res,
                    status,
                    err_msg,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::info!("日志添加失败：{}", e.to_string());
                    }
                };
            });
        }
        _ => return Ok(()),
    }

    Ok(())
}

/// 文件日志
fn file_log(
    req_data: RequestContext,
    now: chrono::NaiveDateTime,
    duration_data: Duration,
    res_data: String,
    err_msg_data: String,
) {
    tracing::info!(
        "\n请求路径:{:?}\n完成时间:{:?}\n消耗时间:{:?}微秒 | {:?}毫秒\n请求数据:{:?}\n响应数据:{}\n错误信息:{:?}\n",
        req_data.path.clone(),
        now,
        duration_data.as_micros(),
        duration_data.as_millis(),
        req_data,
        res_data,
        err_msg_data,
    );
}
