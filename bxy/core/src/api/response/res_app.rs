/// 应用授权验证响应数据结构
pub struct AppAuthRes {
    /// 应用授权码
    pub app_code: String,
    /// 临时授权码
    pub temp_code: String,
}
