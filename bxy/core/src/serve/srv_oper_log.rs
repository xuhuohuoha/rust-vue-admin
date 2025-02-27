use core::time::Duration;

use anyhow::{anyhow, Result};
use byz_entity::core::{sys_oper_log, OperLogEntity, OperLogModel};
use sea_orm::{
    sea_query::Table, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_log::OperLogSearchReq;
use crate::model::app_structs::ToPageData;
use crate::{
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams, RequestContext, UserInfoContext},
    serve::srv_online,
};

#[allow(clippy::too_many_arguments)]
pub async fn db_log(
    duration: Duration,
    ctx: RequestContext,
    ctx_user: UserInfoContext,
    now: chrono::NaiveDateTime,
    api_name: String,
    res: String,
    status: String,
    err_msg: String,
) -> Result<()> {
    let d = duration.as_micros() as i64;
    let db = DB.get_or_init(db_conn).await;
    let (_, m) = srv_online::check_online(Some(db), &ctx_user.token).await;
    let user = match m {
        Some(x) => x,
        None => return Ok(()),
    };
    let operator_type = match ctx.method.as_str() {
        "GET" => "1",    // 查询
        "POST" => "2",   // 新增
        "PUT" => "3",    // 修改
        "DELETE" => "4", // 删除
        _ => "0",        // 其他
    };
    let add_data = sys_oper_log::ActiveModel {
        id: Set(scru128::new_string()),
        time_id: Set(now.and_utc().timestamp()),
        title: Set(api_name),
        business_type: Set("".to_string()),
        method: Set(ctx.path),
        request_method: Set(ctx.method),
        operator_type: Set(operator_type.to_string()),
        oper_name: Set(ctx_user.u_code),
        oper_url: Set(ctx.ori_uri),
        oper_ip: Set(user.ip),
        oper_location: Set(user.login_location),
        oper_param: Set(if ctx.data.len() > 10000 {
            "数据太长不保存".to_string()
        } else {
            ctx.data
        }),
        json_result: Set(if res.len() > 65535 {
            "数据太长不保存".to_string()
        } else {
            res
        }),
        path_param: Set(ctx.path_params),
        status: Set(status),
        error_msg: Set(err_msg),
        duration: Set(d),
        oper_time: Set(now),
    };
    OperLogEntity::insert(add_data)
        .exec(db)
        .await
        .expect("oper_log_add error");
    Ok(())
}

/// 清空登录日志
pub async fn clean() -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    let stmt = Table::truncate().table(sys_oper_log::Entity).to_owned();
    let db_backend = db.get_database_backend();
    db.execute(db_backend.build(&stmt)).await?;
    Ok("登录日志已清空".to_string())
}

/// 硬删除日志
pub async fn delete(req: DeleteReq) -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    match OperLogEntity::delete_many()
        .filter(sys_oper_log::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败，数据不存在",)),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 获取日志列表
pub async fn find_all(
    page_params: PageParams,
    req: OperLogSearchReq,
) -> Result<PageData<OperLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(10);

    // 条件处理
    let mut q = OperLogEntity::find();
    if let Some(x) = req.oper_name {
        if !x.is_empty() {
            q = q.filter(sys_oper_log::Column::OperName.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_oper_log::Column::TimeId)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
