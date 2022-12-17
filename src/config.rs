use std::env;

use error_chain::error_chain;
use nacos_sdk::api::{naming::{NamingServiceBuilder, ServiceInstance, NamingService}, props::ClientProps, constants};
use schemars::schema::RootSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::{error, info};

pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

error_chain! {
    foreign_links {
        Io(std::io::Error);
        NacosError(nacos_sdk::api::error::Error);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    pub active: String,
}
// 用来接收application.yml解析结果
#[derive(Serialize, Deserialize, Debug)]
pub struct EnvConfig {
    pub profiles: Profiles,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Mysql {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub database: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Nacos {
    pub server_addr: String,
    pub namespace: String,
    pub group: String,
    pub data_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub log_level: u8,
}
// 用来接收application-dev.yml解析结果
#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    // 解析对应的mysql配置
    pub mysql: Mysql,
    // Nacos
    pub nacos: Nacos,
    // 服务器配置
    pub server: Server,
}

// 加载指定配置文件
fn load_config<T>(path: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let pwd = env::current_dir().unwrap_or_else(|_| panic!("Can not get current directory path."));

    let file = pwd.join(path);

    if !file.exists() {
        panic!(
            "Global configuration file {} not exists.",
            file.into_os_string().into_string().unwrap()
        )
    }
    // 1.通过std::fs读取配置文件内容
    // 2.通过serde_yaml解析读取到的yaml配置转换成json对象
    match serde_yaml::from_str::<RootSchema>(
        &std::fs::read_to_string(path).unwrap_or_else(|_| panic!("failure read file {}", path)),
    ) {
        Ok(root_schema) => {
            // 通过serde_json把json对象转换指定的model
            let data =
                serde_json::to_string_pretty(&root_schema).expect("failure to parse RootSchema");
            // debug!("json config string: {}", data);
            let config = serde_json::from_str::<T>(&data).unwrap();
            // .unwrap_or_else(|_| panic!("failure to format json str {}", &data));
            // 返回格式化结果
            Some(config)
        }
        Err(err) => {
            // 记录日志
            info!("{}", err);
            // 返回None
            None
        }
    }
}

// 加载配置文件 application.yaml
pub fn load_env_config() -> Option<EnvConfig> {
    load_config::<EnvConfig>("application.yaml")
}

// 根据环境加载 application-{}.yaml 文件
pub fn load_global_config_from_env(active: String) -> Option<GlobalConfig> {
    let path = format!("application-{}.yaml", active);
    load_config::<GlobalConfig>(&path)
}

pub fn load_global_config() -> Option<GlobalConfig> {
    if let Some(env_config) = load_env_config() {
        load_global_config_from_env(env_config.profiles.active)
    } else {
        None
    }
}

pub fn register_nacos() -> Result<()> {
    let env_config = load_env_config().unwrap();

    // 加载配置
    let global_config = match load_global_config() {
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
    let client_props = ClientProps::new()
        .server_addr(global_config.nacos.server_addr)
        // Attention! "public" is "", it is recommended to customize the namespace with clear meaning.
        .namespace(global_config.nacos.namespace)
        .app_name(PACKAGE_NAME);

    
    let naming_service = NamingServiceBuilder::new(client_props).build()?;

    let service_instance1 = ServiceInstance {
        ip: "127.0.0.1".to_string(),
        port: 9092,
        weight: 1.0,
        ..Default::default()
    };
    tracing::info!("Register service instance {}:{}", global_config.server.host, global_config.server.port);
    let _register_instance_ret = naming_service.batch_register_instance(
        PACKAGE_NAME.to_string(),
        Some(constants::DEFAULT_GROUP.to_string()),
        vec![service_instance1],
    );
    // sleep(Duration::from_millis(111)).await;

    tracing::debug!("Get all instances");
    let instances_ret = naming_service.get_all_instances(
        PACKAGE_NAME.to_string(),
        Some(constants::DEFAULT_GROUP.to_string()),
        Vec::default(),
        false,
    );
    match instances_ret {
        Ok(instances) => tracing::info!("get_all_instances {:?}", instances),
        Err(err) => tracing::error!("naming get_all_instances error {:?}", err),
    }

    Ok(())
}
