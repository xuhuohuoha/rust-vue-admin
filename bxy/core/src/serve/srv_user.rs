//!
//! Blunka BPM 用户功能业务接口
//!

use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use hyper::HeaderMap;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbConn, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_login::LoginReq;
use crate::api::request::req_user::{
    ResetPasswordReq, UpdatePasswordReq, UserReq, UserSearchReq, UserStatusReq,
};
use crate::model::app_structs::{PageData, PageParams, ToPageData};
use crate::model::dict_structs::{DataStatus, OperResult};
use crate::utils::{EncryptUtils, RandUtils};
use byz_entity::core::{
    sys_user, sys_user_auth, sys_user_duty, sys_user_org, sys_user_post, sys_user_role,
    UserAuthEntity, UserDutyEntity, UserEntity, UserModel, UserOrgEntity, UserPostEntity,
    UserRoleEntity,
};

use super::srv_online;
use super::utils::jwt::{self, AuthBody, AuthPayload};
use super::utils::web_utils::get_client_info;

/// 新建用户
pub async fn add(db: &DbConn, req: UserReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let uid = scru128::new_string().to_uppercase();
    let now: NaiveDateTime = Local::now().naive_local();
    let salt = RandUtils::rand_str(10);
    let password = EncryptUtils::encrypt_password(&req.upwd, &salt);
    sys_user::ActiveModel {
        id: Set(id.clone()),
        u_id: Set(uid),
        create_by: Set(op_user.to_string()),
        created_at: Set(now),
        version: Set(1),
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(req.remark),
        ucode: Set(req.ucode),
        uname: Set(req.uname),
        upwd: Set(password),
        sex: Set(req.sex),
        salt: Set(salt),
        email: Set(req.email),
        qq: Set(req.qq),
        webchat: Set(req.webchat),
        phone: Set(req.phone),
        pin: Set(req.pin),
        pass: Set(req.pass),
        avatar: Set(Some(req.avatar)),
        ext1: Set(req.ext1),
        ext2: Set(req.ext2),
        ext3: Set(req.ext3),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

/// 编辑用户
pub async fn edit(db: &DbConn, req: UserReq, op_user: &str) -> Result<String> {
    let id = req.id;
    // 更新用户信息
    UserEntity::update_many()
        .col_expr(sys_user::Column::UId, Expr::value(req.u_id))
        .col_expr(sys_user::Column::Ucode, Expr::value(req.ucode))
        .col_expr(sys_user::Column::Uname, Expr::value(req.uname))
        .col_expr(sys_user::Column::Sex, Expr::value(req.sex))
        .col_expr(sys_user::Column::Email, Expr::value(req.email))
        .col_expr(sys_user::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_user::Column::Qq, Expr::value(req.qq))
        .col_expr(sys_user::Column::Webchat, Expr::value(req.webchat))
        .col_expr(sys_user::Column::Phone, Expr::value(req.phone))
        .col_expr(sys_user::Column::Pin, Expr::value(req.pin))
        .col_expr(sys_user::Column::Pass, Expr::value(req.pass))
        .col_expr(sys_user::Column::Avatar, Expr::value(req.avatar))
        .col_expr(sys_user::Column::Ext1, Expr::value(req.ext1))
        .col_expr(sys_user::Column::Ext2, Expr::value(req.ext2))
        .col_expr(sys_user::Column::Ext3, Expr::value(req.ext3))
        .col_expr(
            sys_user::Column::UpdatedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_user::Column::UpdateBy, Expr::value(op_user))
        .filter(sys_user::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok("用户数据更新成功".to_string())
}

/// 软删除用户
pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    // 更新用户信息
    UserEntity::update_many()
        .col_expr(
            sys_user::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_user::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_user::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_user::Column::Id.eq(id))
        .exec(db)
        .await?;
    delete_user_related(db, op_user).await?;
    Ok(true)
}

/// 软删除用户
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    // 更新用户信息
    UserEntity::update_many()
        .col_expr(
            sys_user::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_user::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_user::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_user::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    delete_user_related(db, op_user).await?;
    Ok(true)
}

/// 硬删除用户
pub async fn delete_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<String> {
    let d = UserEntity::delete_many()
        .filter(sys_user::Column::Id.eq(id))
        .exec(db)
        .await?;
    delete_user_related(db, op_user).await?;
    match d.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在.")),
        _i => Ok("成功删除数据".to_string()),
    }
}

/// 硬删除用户
pub async fn delete(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<String> {
    let d = UserEntity::delete_many()
        .filter(sys_user::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    delete_user_related(db, op_user).await?;
    match d.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 查询用户（分页）
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: UserSearchReq,
) -> Result<PageData<UserModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = UserEntity::find();

    if let Some(x) = req.u_id {
        if !x.is_empty() {
            q = q.filter(sys_user::Column::UId.eq(&x));
        }
    }
    if let Some(x) = req.status {
        if !x.is_empty() {
            q = q.filter(sys_user::Column::Status.eq(&x));
        }
    }
    if let Some(x) = req.ucode {
        if !x.is_empty() {
            q = q.filter(sys_user::Column::Ucode.contains(&x));
        }
    }
    if let Some(x) = req.uname {
        if !x.is_empty() {
            q = q.filter(sys_user::Column::Uname.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_user::Column::Ucode)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}

/// 查询用户（根据 ID）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<UserModel> {
    match UserEntity::find()
        .filter(sys_user::Column::DeletedAt.is_null())
        .filter(sys_user::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("用户不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询用户（根据 用户ID）
pub async fn find_by_uid(db: &DbConn, u_id: &str) -> Result<UserModel> {
    match UserEntity::find()
        .filter(sys_user::Column::DeletedAt.is_null())
        .filter(sys_user::Column::UId.eq(u_id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("用户不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询用户（根据 用户账号）
pub async fn find_by_ucode(db: &DbConn, ucode: &str) -> Result<UserModel> {
    match UserEntity::find()
        .filter(sys_user::Column::DeletedAt.is_null())
        .filter(sys_user::Column::Ucode.eq(ucode))
        .one(db)
        .await?
    {
        None => Err(anyhow!("用户不存在")),
        Some(u) => Ok(u),
    }
}

/// 用户登录
pub async fn login(
    db: &DatabaseConnection,
    login_req: LoginReq,
    header: HeaderMap,
) -> Result<AuthBody> {
    let mut msg = "登录成功".to_string();
    let mut status = OperResult::Success.to_string();
    // 验证验证码
    if EncryptUtils::encrypt_password(&login_req.code, "") != login_req.uuid {
        msg = "验证码错误".to_string();
        status = OperResult::Failed.to_string();
        set_login_info(
            header,
            "".to_string(),
            login_req.usercode.clone(),
            msg.clone(),
            status.clone(),
            None,
            None,
        )
        .await;
        return Err(anyhow!("验证码错误"));
    }
    // 根据用户名获取用户信息
    let user = match UserEntity::find()
        .filter(sys_user::Column::Ucode.eq(login_req.usercode.clone()))
        .one(db)
        .await?
    {
        Some(user) => {
            if user.status == DataStatus::Disabled.to_string() {
                msg = "用户已被禁用".to_string();
                status = OperResult::Failed.to_string();
                set_login_info(
                    header,
                    "".to_string(),
                    login_req.usercode.clone(),
                    msg.clone(),
                    status.clone(),
                    None,
                    None,
                )
                .await;
                return Err(anyhow!("用户已被禁用"));
            } else {
                user
            }
        }
        None => {
            msg = "用户不存在".to_string();
            status = OperResult::Failed.to_string();
            set_login_info(
                header,
                "".to_string(),
                login_req.usercode.clone(),
                msg.clone(),
                status.clone(),
                None,
                None,
            )
            .await;
            return Err(anyhow!("用户不存在"));
        }
    };
    //  验证密码是否正确
    if EncryptUtils::encrypt_password(&login_req.password, &user.salt) != user.upwd {
        msg = "密码错误".to_string();
        status = OperResult::Failed.to_string();
        set_login_info(
            header,
            "".to_string(),
            login_req.usercode.clone(),
            msg.clone(),
            status.clone(),
            None,
            None,
        )
        .await;
        return Err(anyhow!("密码不正确"));
    };
    // 注册JWT
    let claims = AuthPayload {
        u_id: user.u_id.clone(),
        u_code: login_req.usercode.clone(), // 用户名
    };
    let token_id = scru128::new_string();
    let token = jwt::authorize(claims.clone(), token_id.clone())
        .await
        .unwrap();

    // 写入登录日志
    set_login_info(
        header,
        user.id.to_string(),
        login_req.usercode.clone(),
        msg.clone(),
        status.clone(),
        Some(token_id),
        Some(token.clone()),
    )
    .await;

    Ok(token)
}

/// 设置登录日志
pub async fn set_login_info(
    header: HeaderMap,
    u_id: String,
    user: String,
    msg: String,
    status: String,
    token_id: Option<String>,
    token: Option<AuthBody>,
) {
    let u = get_client_info(header).await;
    // 写入登录日志
    let u2 = u.clone();
    let status2 = status.clone();
    // 如果成功，写入在线日志
    if status == OperResult::Success.to_string() {
        if let (Some(token_id), Some(token)) = (token_id, token) {
            srv_online::add(u, u_id.as_str(), token_id.as_str(), token.clone().exp).await;
        }
    };
    tokio::spawn(async move {
        super::srv_login_log::add(u2, user, msg, status2, "用户模块".to_string()).await;
    });
}

/// 重置密码
pub async fn reset_password(db: &DatabaseConnection, req: ResetPasswordReq) -> Result<String> {
    let salt = RandUtils::rand_str(10);
    let password = EncryptUtils::encrypt_password(&req.new_password, &salt);
    let now: NaiveDateTime = Local::now().naive_local();
    let txn = db.begin().await?;
    // 更新用户信息
    UserEntity::update_many()
        .col_expr(sys_user::Column::Upwd, Expr::value(password))
        .col_expr(sys_user::Column::Salt, Expr::value(salt))
        .col_expr(sys_user::Column::UpdatedAt, Expr::value(now))
        .filter(sys_user::Column::Id.eq(req.u_id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "密码更新成功".to_string();

    Ok(res)
}

/// 修改密码
pub async fn update_password(db: &DatabaseConnection, req: UpdatePasswordReq) -> Result<String> {
    match UserEntity::find()
        .filter(sys_user::Column::Id.eq(&req.u_id))
        .one(db)
        .await?
    {
        None => return Err(anyhow!("用户不存在.")),
        Some(x) => {
            let pwd = EncryptUtils::encrypt_password(&req.old_password, &x.salt);
            if pwd == x.upwd {
                return Err(anyhow!("旧密码错误,请检查重新输入"));
            }
        }
    };
    self::reset_password(
        db,
        ResetPasswordReq {
            u_id: req.u_id,
            new_password: req.new_password,
        },
    )
    .await
}

/// 修改状态
pub async fn change_status(db: &DatabaseConnection, req: UserStatusReq) -> Result<String> {
    let now: NaiveDateTime = Local::now().naive_local();
    // 更新用户信息
    UserEntity::update_many()
        .col_expr(sys_user::Column::Status, Expr::value(req.status))
        .col_expr(sys_user::Column::UpdatedAt, Expr::value(now))
        .filter(sys_user::Column::UId.eq(req.u_id))
        .exec(db)
        .await?;
    let res = "用户状态更新成功".to_string();
    Ok(res)
}

/// 正常
pub async fn enabled(db: &DbConn, u_id: &str) -> Result<bool> {
    self::change_status(
        db,
        UserStatusReq {
            u_id: u_id.to_string(),
            status: DataStatus::Enabled.to_string(),
        },
    )
    .await?;
    Ok(true)
}

/// 禁用
pub async fn disabled(db: &DbConn, u_id: &str) -> Result<bool> {
    self::change_status(
        db,
        UserStatusReq {
            u_id: u_id.to_string(),
            status: DataStatus::Disabled.to_string(),
        },
    )
    .await?;
    Ok(true)
}

/// 删除用户关联的部门、角色、岗位、职务、权限
pub async fn delete_user_related(db: &DbConn, op_user: &str) -> Result<()> {
    UserOrgEntity::delete_many()
        .filter(sys_user_org::Column::UId.eq(op_user))
        .exec(db)
        .await?;
    UserRoleEntity::delete_many()
        .filter(sys_user_role::Column::UId.eq(op_user))
        .exec(db)
        .await?;
    UserPostEntity::delete_many()
        .filter(sys_user_post::Column::UId.eq(op_user))
        .exec(db)
        .await?;
    UserDutyEntity::delete_many()
        .filter(sys_user_duty::Column::UId.eq(op_user))
        .exec(db)
        .await?;
    UserAuthEntity::delete_many()
        .filter(sys_user_auth::Column::UId.eq(op_user))
        .exec(db)
        .await?;
    Ok(())
}
