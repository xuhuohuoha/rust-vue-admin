//! Blunka BPM 菜单功能业务接口
//!
use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use byz_entity::core::{sys_menu, MenuEntity, MenuModel};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_menu::{MenuReq, UserMenuReq};
use crate::api::response::res_menu::{MenuTreeRes, UserMenu};
use crate::config::CONFIG;
use crate::model::app_structs::{PageData, ToPageData};
use crate::model::dict_structs::{DataStatus, MenuType};
use crate::{api::request::req_menu::MenuSearchReq, model::app_structs::PageParams};

use super::srv_user_auth;

/// 新增菜单
pub async fn add(db: &DbConn, req: MenuReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let guid = if req.guid.is_empty() {
        scru128::new_string().to_uppercase()
    } else {
        req.guid.clone()
    };
    let pguid = if req.pguid.is_empty() {
        guid.clone()
    } else {
        req.pguid
    };
    let menu = find_by_guid(db, &guid).await;
    match menu {
        Ok(_) => return Err(anyhow!("当前菜单已存在，请勿重复添加.")),
        Err(_) => {
            let now: NaiveDateTime = Local::now().naive_local();
            sys_menu::ActiveModel {
                id: Set(id.clone()),
                create_by: Set(op_user.to_string()),
                created_at: Set(now),
                guid: Set(guid),
                pguid: Set(pguid),
                icon: Set(req.icon),
                version: Set(1),
                ord: Set(req.ord),
                status: Set(req.status),
                remark: Set(req.remark),
                app: Set(req.app),
                mname: Set(req.mname),
                mtype: Set(req.mtype),
                uio: Set(req.uio),
                path: Set(req.path),
                api: Set(req.api),
                method: Set(req.method),
                opt: Set(req.opt),
                alias: Set(req.alias),
                tbl: Set(req.tbl),
                query: Set(req.query),
                qscript: Set(req.qscript),
                cols: Set(req.cols),
                cscript: Set(req.cscript),
                style: Set(req.style),
                show: Set(req.show),
                comp: Set(req.comp),
                visible: Set(req.visible),
                is_cache: Set(req.is_cache),
                log_method: Set(req.log_method),
                data_cache_method: Set(req.data_cache_method),
                data_scope: Set(req.data_scope),
                i18n: Set(req.i18n),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }
    }
    Ok(id)
}

/// 编辑菜单
pub async fn edit(db: &DbConn, req: MenuReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now: NaiveDateTime = Local::now().naive_local();
    MenuEntity::update_many()
        .col_expr(sys_menu::Column::Guid, Expr::value(req.guid))
        .col_expr(sys_menu::Column::Pguid, Expr::value(req.pguid))
        .col_expr(sys_menu::Column::Icon, Expr::value(req.icon))
        .col_expr(sys_menu::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_menu::Column::Status, Expr::value(req.status))
        .col_expr(sys_menu::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_menu::Column::App, Expr::value(req.app))
        .col_expr(sys_menu::Column::Mname, Expr::value(req.mname))
        .col_expr(sys_menu::Column::Mtype, Expr::value(req.mtype))
        .col_expr(sys_menu::Column::Uio, Expr::value(req.uio))
        .col_expr(sys_menu::Column::Path, Expr::value(req.path))
        .col_expr(sys_menu::Column::Api, Expr::value(req.api))
        .col_expr(sys_menu::Column::Method, Expr::value(req.method))
        .col_expr(sys_menu::Column::Opt, Expr::value(req.opt))
        .col_expr(sys_menu::Column::Alias, Expr::value(req.alias))
        .col_expr(sys_menu::Column::Tbl, Expr::value(req.tbl))
        .col_expr(sys_menu::Column::Query, Expr::value(req.query))
        .col_expr(sys_menu::Column::Qscript, Expr::value(req.qscript))
        .col_expr(sys_menu::Column::Cols, Expr::value(req.cols))
        .col_expr(sys_menu::Column::Cscript, Expr::value(req.cscript))
        .col_expr(sys_menu::Column::Style, Expr::value(req.style))
        .col_expr(sys_menu::Column::Show, Expr::value(req.show))
        .col_expr(sys_menu::Column::Comp, Expr::value(req.comp))
        .col_expr(sys_menu::Column::Visible, Expr::value(req.visible))
        .col_expr(sys_menu::Column::IsCache, Expr::value(req.is_cache))
        .col_expr(sys_menu::Column::LogMethod, Expr::value(req.log_method))
        .col_expr(
            sys_menu::Column::DataCacheMethod,
            Expr::value(req.data_cache_method),
        )
        .col_expr(sys_menu::Column::DataScope, Expr::value(req.data_scope))
        .col_expr(sys_menu::Column::I18n, Expr::value(req.i18n))
        .col_expr(sys_menu::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_menu::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_menu::Column::Version,
            Expr::value(Expr::col(sys_menu::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_menu::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

/// 软删除菜单（根据 ID）
pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    MenuEntity::update_many()
        .col_expr(
            sys_menu::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_menu::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_menu::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_menu::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

/// 软删除菜单（批量 ID）
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: String) -> Result<bool> {
    MenuEntity::update_many()
        .col_expr(sys_menu::Column::Status, Expr::value("2".to_string()))
        .col_expr(
            sys_menu::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_menu::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_menu::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

/// 硬删除菜单（根据 ID）
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match MenuEntity::delete_many()
        .filter(sys_menu::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

/// 硬删除菜单（批量 ID）
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match MenuEntity::delete_many()
        .filter(sys_menu::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 查询菜单（根据 ID）
pub async fn find_by_id(db: &DbConn, id: String) -> Result<MenuModel> {
    match MenuEntity::find()
        .filter(sys_menu::Column::DeletedAt.is_null())
        .filter(sys_menu::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("菜单不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询菜单（根据 GUID）
pub async fn find_by_guid(db: &DbConn, guid: &str) -> Result<MenuModel> {
    match MenuEntity::find()
        .filter(sys_menu::Column::DeletedAt.is_null())
        .filter(sys_menu::Column::Guid.eq(guid))
        .one(db)
        .await?
    {
        None => Err(anyhow!("菜单不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询菜单（根据 PGUID）
pub async fn find_by_pguid(db: &DbConn, pguid: &str) -> Result<MenuModel> {
    match MenuEntity::find()
        .filter(sys_menu::Column::DeletedAt.is_null())
        .filter(sys_menu::Column::Pguid.eq(pguid))
        .one(db)
        .await?
    {
        None => Err(anyhow!("菜单不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询所有菜单（分页）
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: MenuSearchReq,
) -> Result<PageData<MenuModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = MenuEntity::find();
    // filter(sys_menu::Column::Mtype.not_like(MenuType::F.to_string() + "%"));

    // 条件处理
    if let Some(x) = req.guid {
        if !x.is_empty() {
            q = q.filter(sys_menu::Column::Guid.is_in(&x));
        }
    }
    if let Some(x) = req.mname {
        if !x.is_empty() {
            q = q.filter(sys_menu::Column::Mname.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_menu::Column::Ord)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}

/// 查询所有菜单目录（分页列表）
pub async fn find_menu_all(
    db: &DbConn,
    page_params: PageParams,
    req: MenuSearchReq,
) -> Result<PageData<MenuModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = MenuEntity::find().filter(sys_menu::Column::Mtype.eq(MenuType::C.to_string()));

    // 条件处理
    if let Some(x) = req.guid {
        if !x.is_empty() {
            q = q.filter(sys_menu::Column::Guid.is_in(&x));
        }
    }
    if let Some(x) = req.mname {
        if !x.is_empty() {
            q = q.filter(sys_menu::Column::Mname.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_menu::Column::Ord)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}

/// 查询所有菜单功能（分页列表）
pub async fn find_fn_all(
    db: &DbConn,
    page_params: PageParams,
    req: MenuSearchReq,
) -> Result<PageData<MenuModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q =
        MenuEntity::find().filter(sys_menu::Column::Mtype.like(MenuType::F.to_string() + "%"));

    // 条件处理
    if let Some(x) = req.pguid {
        if !x.is_empty() {
            q = q.filter(sys_menu::Column::Pguid.is_in(&x));
        }
    }
    if let Some(x) = req.mname {
        if !x.is_empty() {
            q = q.filter(sys_menu::Column::Mname.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_menu::Column::Ord)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}

/// 查询所有菜单目录（指定应用、指定用户）
pub async fn find_menu_by_uid(db: &DbConn, req: UserMenuReq) -> Result<Vec<MenuTreeRes>> {
    // 获取当前应用下所有菜单
    let app_menus = find_app_menu(db, &req.app_code).await?;
    // 获取当前用户已授权菜单
    let auth_menus = find_auth_menu(db, &req.u_id, "", app_menus.clone()).await?;
    // 获取根目录
    let root_menus = find_root_menu(&app_menus);
    // 生成菜单树
    let menu_tree = root_menus
        .iter()
        .map(|x| create_menu_tree(x.clone(), &auth_menus, &app_menus))
        .collect();
    Ok(menu_tree)
}

/// 查询功能（指定菜单、指定用户）
pub async fn find_fn_by_uid(db: &DbConn, req: UserMenuReq) -> Result<Vec<MenuModel>> {
    // 获取当前菜单下所有功能
    let mut menu_fns = find_menu_fn(db, &req.mcode).await?;
    // 获取当前用户已授权菜单
    let auth_menus = find_auth_menu(db, &req.u_id, &req.mcode, menu_fns.clone()).await?;
    // 移除未授权功能
    menu_fns.retain(|x| !auth_menus.contains(&x.guid));

    Ok(menu_fns)
}

/// 查询授权菜单
pub async fn find_auth_menu(
    db: &DbConn,
    u_id: &str,
    mcode: &str,
    app_menus: Vec<MenuModel>,
) -> Result<Vec<String>> {
    let mut auth_menus: Vec<String> = vec![];
    // 如果是超级用户，则不需要验证权限，直接返回所有菜单
    if CONFIG.system.super_user.contains(&u_id.to_string()) {
        app_menus.iter().for_each(|x| {
            if x.mtype == MenuType::C.to_string() {
                auth_menus.push(x.guid.clone());
            }
        });
    } else {
        // 获取当前用户已授权菜单
        srv_user_auth::find_uid(db, u_id, mcode)
            .await?
            .iter()
            .for_each(|x| auth_menus.push(x.clone()));
    }
    Ok(auth_menus)
}

/// 查询菜单（指定应用）
///
/// 菜单类型为 F 表示的是功能，应予以排除
///
pub async fn find_app_menu(db: &DbConn, app: &str) -> Result<Vec<MenuModel>> {
    let mut q = MenuEntity::find()
        .filter(sys_menu::Column::Mtype.eq(MenuType::C.to_string()))
        .filter(sys_menu::Column::DeletedAt.is_null());
    if !app.is_empty() {
        q = q.filter(sys_menu::Column::App.eq(app));
    }
    match q.order_by_asc(sys_menu::Column::Ord).all(db).await {
        Ok(x) => Ok(x),
        Err(e) => Err(anyhow!(e)),
    }
}

/// 查询根目录（指定应用）
///
/// 菜单类型为 F 表示的是功能，应予以排除
///
pub async fn find_app_root_menu(db: &DbConn, app: &str) -> Result<Vec<MenuModel>> {
    let mut q = MenuEntity::find()
        .filter(sys_menu::Column::Mtype.eq(MenuType::C.to_string()))
        .filter(sys_menu::Column::DeletedAt.is_null());
    if !app.is_empty() {
        q = q.filter(sys_menu::Column::App.eq(app));
    }
    match q.order_by_asc(sys_menu::Column::Ord).all(db).await {
        Ok(x) => Ok(x.iter().filter(|&x| x.guid == x.pguid).cloned().collect()),
        Err(e) => Err(anyhow!(e)),
    }
}

/// 查询所有Api
///
/// # 参数
/// - is_menu：是否仅菜单Api，true仅查询菜单目录Api，false查询所有Api
/// - is_fn：是否仅功能Api，true仅查询菜单功能Api，false查询所有Api
/// - is_enabled：是否仅可用Api，true仅查询可用Api，false查询所有Api
pub async fn find_api(
    db: &DbConn,
    is_menu: bool,
    is_fn: bool,
    is_enabled: bool,
) -> Result<Vec<MenuModel>> {
    let mut q = MenuEntity::find()
        .filter(sys_menu::Column::DeletedAt.is_null())
        .filter(sys_menu::Column::Api.is_not_null());
    if is_menu {
        q = q.filter(sys_menu::Column::Mtype.eq(MenuType::C.to_string()));
    };
    if is_fn {
        q = q.filter(sys_menu::Column::Mtype.ne(MenuType::C.to_string()));
    };
    if is_enabled {
        q = q.filter(sys_menu::Column::Status.eq(DataStatus::Enabled.to_string()));
    };
    match q.all(db).await {
        Ok(x) => Ok(x),
        Err(e) => Err(anyhow!(e)),
    }
}

/// 查询指定菜单功能
pub async fn find_menu_fn(db: &DbConn, mcode: &str) -> Result<Vec<MenuModel>> {
    match MenuEntity::find()
        .filter(sys_menu::Column::Pguid.eq(mcode))
        .filter(sys_menu::Column::Mtype.like(MenuType::F.to_string() + "%"))
        .filter(sys_menu::Column::DeletedAt.is_null())
        .order_by_asc(sys_menu::Column::Ord)
        .all(db)
        .await
    {
        Ok(x) => Ok(x),
        Err(e) => Err(anyhow!(e)),
    }
}

/// 返回根目录
pub fn find_root_menu(app_menus: &[MenuModel]) -> Vec<MenuModel> {
    app_menus
        .iter()
        .filter(|&x| x.guid == x.pguid)
        .cloned()
        .collect()
}

/// 返回直接子目录
pub fn find_child_menu(menu: MenuModel, app_menus: &[MenuModel]) -> Vec<MenuModel> {
    app_menus
        .iter()
        .filter(|&x| menu.guid == x.pguid && x.guid != x.pguid)
        .cloned()
        .collect()
}

pub async fn find_app_menu_children_tree(db: &DbConn, app_code: &str, guid: &str) -> MenuTreeRes {
    // 获取当前应用下所有菜单
    let app_menus = find_app_menu(db, app_code).await.unwrap();
    // 获取当前目录
    let root_menu = find_by_guid(db, guid).await.unwrap();
    // 获取当前应用菜单GUID集合
    let auth_menus = app_menus.iter().map(|x| x.guid.clone()).collect();
    // 生成菜单树
    create_menu_tree(root_menu, &auth_menus, &app_menus)
}

/// 创建单个菜单树
pub fn create_menu_tree(
    menu: MenuModel,
    auth_menus: &Vec<String>,
    app_menus: &Vec<MenuModel>,
) -> MenuTreeRes {
    let menu2 = menu.clone();
    let model: UserMenu = UserMenu {
        guid: menu.guid.clone(),
        pguid: menu.pguid.clone(),
        ord: menu.ord,
        mname: menu.mname,
        mtype: menu.mtype,
        uio: menu.uio,
        path: menu.path,
        api: menu.api,
        method: menu.method,
        opt: menu.opt,
        alias: menu.alias,
        tbl: menu.tbl,
        query: menu.query,
        qscript: menu.qscript,
        cols: menu.cols,
        cscript: menu.cscript,
        icon: menu.icon,
        style: menu.style,
        show: menu.show,
        comp: menu.comp,
        visible: menu.visible,
    };
    let mut valid: bool = false;

    // 当前菜单为已授权菜单，则直接标记为有效
    if auth_menus.contains(&menu.guid) {
        valid = true;
    }

    let children = find_child_menu(menu2, app_menus)
        .iter()
        .map(|x| create_menu_tree(x.clone(), auth_menus, app_menus))
        .collect();

    MenuTreeRes {
        menu: model,
        valid,
        children: Some(children),
    }
}
