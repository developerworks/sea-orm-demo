mod config;
use error_chain::error_chain;
use sea_orm::DatabaseConnection;
use std::env;
use tracing_subscriber::{filter, fmt, prelude::*, reload};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use config::GlobalConfig;
use tracing::{self, debug, error, Level};

use ::entity::trauma_patient;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        NacosError(nacos_sdk::api::error::Error);
    }
}

#[allow(unused)]
// 包名称, Cargo.toml 的 [package.name] 名字, cargo new 时的名字, 作为服务注册名称向 Nacos 注册
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> Result<()> {
    // 日志订阅

    let filter = filter::LevelFilter::DEBUG;
    let (filter, reload_handle) = reload::Layer::new(filter);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();

    let env_config = config::load_env_config().unwrap();

    // 加载配置
    let global_config = match config::load_global_config() {
        None => {
            error!("Can not load global configuration file");
            panic!("Missing configuration file")
        }
        Some(mut conf) => {
            // 替换 Profile
            conf.nacos.data_id = conf.nacos.data_id[..].replace("{}", &env_config.profiles.active);
            conf
        }
    };

    let filter = match global_config.server.log_level {
        0 => filter::LevelFilter::from_level(Level::TRACE),
        1 => filter::LevelFilter::from_level(Level::DEBUG),
        2 => filter::LevelFilter::from_level(Level::INFO),
        3 => filter::LevelFilter::from_level(Level::WARN),
        4 => filter::LevelFilter::from_level(Level::ERROR),
        _ => filter::LevelFilter::from_level(Level::INFO),
    };
    // 日志等级依据配置修改
    _ = reload_handle.modify(|f| *f = filter);

    // 注册自身
    _ = config::register_nacos();

    // 调试环境变量,需设置日志等级为 DEBUG
    debug_envs();
    debug_configuration(&global_config);

    tracing::info!(
        "Server is running on {}:{}, log level: {}",
        global_config.server.host,
        global_config.server.port,
        filter
    );

    // TOTO:: 获取数据库连接, 注入全局状态

    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((global_config.server.host, global_config.server.port))?
    .run();

    _ = server.await;

    // let res = test().await;

    // let m = match res {
    //     Some(created) => created,
    //     None => activity::Model {
    //         id: 1,
    //         created_at: DateTimeUtc::from(DateTime::parse_from_rfc3339("1970-01-01T00:00:00+08:00").unwrap()),
    //     }
    // };
    // let datetime = format_datetime(m.created_at);

    // println!("now: {}", datetime);

    Ok(())
}

// fn format_datetime(dt: DateTimeUtc) -> String {
//     format!("{}", dt.with_timezone(&chrono::Local).format("%Y-%m-%d %T"))
// }

// async fn test() -> Option<activity::Model> {
//     let db: DatabaseConnection = Database::connect("mysql://root:root@localhost/trauma")
//         .await
//         .unwrap();
//     Activity::find_by_id(2).one(&db).await.unwrap()
// }

fn debug_envs() {
    let cargo_manifest_dir: String = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
        error!("Can not get env:  CARGO_MANIFEST_DIR");
        panic!();
    });
    debug!("[Env] cargo_manifest_dir: {}", cargo_manifest_dir);
    debug!("[Env] package name      : {}", PACKAGE_NAME);
}

/**
 * 调试配置
 */
#[rustfmt::skip]
fn debug_configuration(conf: &GlobalConfig) {
    debug!("[Configuration] mysql host        : {}", conf.mysql.host);
    debug!("[Configuration] mysql port        : {}", conf.mysql.port);
    debug!("[Configuration] mysql username    : {}", conf.mysql.username);
    debug!("[Configuration] mysql password    : {}", conf.mysql.password);
    debug!("[Configuration] mysql database    : {}", conf.mysql.database);
    debug!("[Configuration] nacos server_addr : {}", conf.nacos.server_addr);
    debug!("[Configuration] nacos namespace   : {}", conf.nacos.namespace);
    debug!("[Configuration] nacos group       : {}", conf.nacos.group);
    debug!("[Configuration] nacos data_id     : {}", conf.nacos.data_id);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    tracing::info!("run manual_hello on /hey");
    HttpResponse::Ok().body("Hey there!")
}
