//! Blunka BPM Service
//!
//! # 描述
//! 伯伦卡业务流程平台核心服务，用于支持核心业务，包括：功能体系、用户体系、授权体系、日志体系、配置体系等。
//!

pub mod srv_adtion;
pub mod srv_adtion_ex;
pub mod srv_app;
pub mod srv_app_auth;
pub mod srv_col_auth;
pub mod srv_datasource;
pub mod srv_dict;
pub mod srv_dict_data;
pub mod srv_dql;
pub mod srv_duty;
pub mod srv_login_log;
pub mod srv_master_detail;
pub mod srv_menu;
pub mod srv_online;
pub mod srv_oper_log;
pub mod srv_org;
pub mod srv_post;
pub mod srv_role;
pub mod srv_row_auth;
pub mod srv_server;
pub mod srv_tree;
pub mod srv_update_log;
pub mod srv_user;
pub mod srv_user_api;
pub mod srv_user_auth;
pub mod srv_user_duty;
pub mod srv_user_org;
pub mod srv_user_post;
pub mod srv_user_role;

pub mod orm;
pub mod utils;

use anyhow::Result;
use axum::extract::Multipart;
use captcha_rust::Captcha;
use tokio::{fs, io::AsyncWriteExt};

use crate::{config::CONFIG, model::app_structs::CaptchaImage, utils::EncryptUtils};

/// 获取验证码
pub fn get_captcha() -> CaptchaImage {
    let captcha = Captcha::new(4, 130, 40);
    let uuid = EncryptUtils::encrypt_password(&captcha.text, "");
    CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    }
}

/// 获取文件类型
fn get_file_type(content_type: &str) -> String {
    match content_type {
        "image/jpeg" => ".jpg".to_string(),
        "image/png" => ".png".to_string(),
        "image/gif" => ".gif".to_string(),
        _ => "".to_string(),
    }
}

/// 上传相关
pub async fn upload_file(mut multipart: Multipart) -> Result<String> {
    if let Some(field) = multipart.next_field().await? {
        let content_type = field
            .content_type()
            .map(ToString::to_string)
            .unwrap_or_else(|| "".to_string());
        let old_url = field
            .file_name()
            .map(ToString::to_string)
            .unwrap_or_else(|| "".to_string());
        let file_type = get_file_type(&content_type);
        let bytes = field.bytes().await?;
        let now = chrono::Local::now();
        let file_path_t: String =
            CONFIG.web.upload_dir.clone() + "/" + &now.format("%Y-%m").to_string();
        let url_path_t = CONFIG.web.upload_url.clone() + "/" + &now.format("%Y-%m").to_string();
        fs::create_dir_all(&file_path_t).await?;
        let file_name = now.format("%d").to_string() + "-" + &scru128::new_string() + &file_type;
        let file_path = file_path_t + "/" + &file_name;
        let url_path = url_path_t + "/" + &file_name;
        let mut file = fs::File::create(&file_path).await?;
        file.write_all(&bytes).await?;
        if !old_url.is_empty() {
            self::delete_file(&old_url).await;
        }
        Ok(url_path)
    } else {
        Err(anyhow::anyhow!("上传文件失败"))
    }
}

/// 删除文件
pub async fn delete_file(file_path: &str) {
    let path = file_path.replace(&CONFIG.web.upload_url, &CONFIG.web.upload_dir);
    match fs::remove_file(&path).await {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("删除文件失败:{}", path);
        }
    }
}
