use axum::{http::StatusCode, response::IntoResponse, Router};
use byz_migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;

use tower_http::{
    compression::{predicate::NotForContentType, CompressionLayer, DefaultPredicate, Predicate},
    cors::CorsLayer,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{self, v1::ApiDoc},
    config::CONFIG,
    serve::utils::ApiUtils,
    utils::BxyEnv,
};

/// 应用启动入口
///
/// # 描述
/// Blunka BPM 平台入口
///
/// # 参数
/// - router：业务系统路由
///
#[tokio::main]
async fn run(router: &Router) -> anyhow::Result<()> {
    // 设置日志级别
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", &CONFIG.log.log_level);
    }
    // tracing_subscriber::fmt::init();

    // env 环境变量加载完成
    dotenv::dotenv().ok();

    // 加载数据库配置（仅用于 Migrator）
    let db_url = env::var("DATABASE_URL").expect(".env 文件中未设置 DATABASE_URL.");
    let host = env::var("HOST").expect(".env 文件中未设置 HOST.");
    let port = env::var("PORT").expect(".env 文件中未设置 PORT.");
    let server_url = format!("{host}:{port}");

    // 连接数据库
    let conn = Database::connect(db_url)
        .await
        .expect("--------------- Database connection failed ---------------");

    // 启动 Cli 自动化部署
    Migrator::up(&conn, None).await.unwrap();

    // let state = AppState::new

    // 系统环境变量
    BxyEnv::setup();

    // 系统变量设置
    let log_env = BxyEnv::get_log_level();

    //  日志设置
    let format = BxyEnv::get_log_format();

    // 文件输出
    let file_appender = tracing_appender::rolling::hourly(&CONFIG.log.dir, &CONFIG.log.file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 标准控制台输出
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let logger = Registry::default()
        .with(EnvFilter::from_default_env().add_directive(log_env.into()))
        .with(
            fmt::Layer::default()
                .with_writer(std_non_blocking)
                .event_format(format.clone())
                .pretty(),
        )
        .with(
            fmt::Layer::default()
                .with_writer(non_blocking)
                .event_format(format),
        );
    tracing::subscriber::set_global_default(logger).unwrap();

    // api全局初始化
    ApiUtils::init_all_api().await;

    // // 定时任务初始化
    // tasks::timer_task_init().await.expect("定时任务初始化失败");

    // 顺序不对可能会导致数据丢失，无法在某些位置获取数据
    // let static_files_service = get_service(
    //     ServeDir::new(&CONFIG.web.dir)
    //         .not_found_service(handler_404.into_service())
    //         .append_index_html_on_directories(true),
    // );

    //  跨域
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .fallback(handler_404)
        // Api 文档入口
        .merge(SwaggerUi::new("/swagger-ui").url("/v1/api-docs/openapi.json", ApiDoc::openapi()))
        // Api 接口入口
        .nest(
            &CONFIG.server.api_prefix,
            api::api_v1().merge(router.clone()),
        )
        .layer(cors);

    // 启用压缩传输
    let app = match &CONFIG.server.content_gzip {
        true => {
            //  开启压缩后 SSE 数据无法返回  text/event-stream 单独处理不压缩
            let predicate =
                DefaultPredicate::new().and(NotForContentType::new("text/event-stream"));
            app.layer(CompressionLayer::new().compress_when(predicate))
        }
        false => app,
    };

    // let app = app.layer(cors);
    // match CONFIG.server.ssl {
    //     true => {
    //         let rustls_config = rustls_server_config();
    //         let tls_acceptor = TlsAcceptor::from(rustls_config);
    //         let tcp_listener = tokio::net::TcpListener::bind(&CONFIG.server.address).await.unwrap();

    //         pin_mut!(tcp_listener);
    //         loop {
    //             let tower_service = app.clone();
    //             let tls_acceptor = tls_acceptor.clone();

    //             // Wait for new tcp connection
    //             let (cnx, addr) = tcp_listener.accept().await.unwrap();

    //             tokio::spawn(async move {
    //                 // Wait for tls handshake to happen
    //                 let Ok(stream) = tls_acceptor.accept(cnx).await else {
    //                     error!("error during tls handshake connection from {}", addr);
    //                     return;
    //                 };

    //                 // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
    //                 // `TokioIo` converts between them.
    //                 let stream = hyper_util::rt::TokioIo::new(stream);

    //                 // Hyper has also its own `Service` trait and doesn't use tower. We can use
    //                 // `hyper::service::service_fn` to create a hyper `Service` that calls our app
    //                 // through `tower::Service::call`.
    //                 let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
    //                     // We have to clone `tower_service` because hyper's `Service` uses `&self`
    //                     // whereas tower's `Service` requires `&mut self`.
    //                     //
    //                     // We don't need to call `poll_ready` since `Router` is always ready.
    //                     tower_service.clone().call(request)
    //                 });

    //                 let ret = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
    //                     .serve_connection_with_upgrades(stream, hyper_service)
    //                     .await;

    //                 if let Err(err) = ret {
    //                     warn!("error serving connection from {}: {}", addr, err);
    //                 }
    //             });
    //         }
    //     }

    //     false => {
    //         let listener = tokio::net::TcpListener::bind(&CONFIG.server.address).await.unwrap();
    //         axum::serve(listener, app).await.unwrap();
    //     }
    // }
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/// 请求 404 处理
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

/// 请求 500 处理
// async fn handler_500() -> impl IntoResponse {
//     (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
// }

// fn rustls_server_config() -> Arc<ServerConfig> {
//     let mut key_reader = BufReader::new(File::open(&CONFIG.cert.key).unwrap());
//     let mut cert_reader = BufReader::new(File::open(&CONFIG.cert.cert).unwrap());

//     let key = PrivateKey(pkcs8_private_keys(&mut key_reader).unwrap().remove(0));
//     let certs = certs(&mut cert_reader).unwrap().into_iter().map(Certificate).collect();

//     let mut config = ServerConfig::builder()
//         // .with_safe_defaults()
//         .with_no_client_auth()
//         .with_single_cert(certs, key)
//         .expect("bad certificate/key");

//     config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

//     Arc::new(config)
// }

/// 应用入口
/// 提供给外部调用
pub fn main(router: Router) {
    let result = run(&router);

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
