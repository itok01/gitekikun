use serde::{Deserialize, Serialize};

use super::config::*;

// データベースに接続
pub fn connect_database() -> mongodb::Database {
    // 設定を読み込む
    let config = load_config();

    // データベースサーバーに接続
    let database_uri = format!(
        "mongodb://{}:{}@{}",
        config.database_username, config.database_password, config.database_host
    );
    let client = mongodb::Client::with_uri_str(database_uri.as_str()).unwrap();

    // データベースに接続
    let database = client.database(config.database_name.as_str());
    database
}

// データベースの初期化
pub fn database_init(database: &mongodb::Database) {
    // 必要なコレクション
    let necessary_collection_names: Vec<&str> = vec!["device"];

    // 存在するコレクション
    let collection_names = database.list_collection_names(None).unwrap();

    // delete_meコレクションがあれば消す
    if collection_names.contains(&"delete_me".to_string()) {
        match database.collection("delete_me").drop(None) {
            Ok(_) => println!("Deleted \"delete_me\" collection."),
            Err(e) => println!("{}", e),
        }
    }

    // 足りないコレクションを作成
    for necessary_collection_name in necessary_collection_names {
        if !collection_names.contains(&necessary_collection_name.to_string()) {
            match database.create_collection(
                necessary_collection_name,
                mongodb::options::CreateCollectionOptions::default(),
            ) {
                Ok(_) => println!("Created \"{}\" collection.", necessary_collection_name),
                Err(e) => println!("{}", e),
            }
        }
    }
}

pub fn add_giteki(database: &mongodb::Database, device: GitekiDevice) {
    let device_collection = database.collection("device");
    if let bson::Bson::Document(device_bson) = bson::to_bson(&device).unwrap() {
        device_collection.insert_one(device_bson, None).ok();
    }
}

pub fn is_giteki_exists(database: &mongodb::Database, device: &GitekiDevice) -> bool {
    let mut result = false;
    let device_collection = database.collection("device");
    if let bson::Bson::Document(device_bson) = bson::to_bson(&device).unwrap() {
        if let Some(_) = device_collection.find_one(device_bson, None).unwrap() {
            result = true;
        }
    }
    result
}

#[derive(Deserialize, Serialize)]
pub struct GitekiDevice {
    #[serde(rename = "techCode")]
    pub tech_code: String,
    pub number: String,
    pub date: String,
    pub name: String,
    #[serde(rename = "radioEquipmentCode")]
    pub radio_equipment_code: String,
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "elecWave")]
    pub elec_wave: String,
    #[serde(rename = "spuriousRules")]
    pub spurious_rules: String,
    #[serde(rename = "bodySar")]
    pub body_sar: String,
    pub note: String,
    #[serde(rename = "organName")]
    pub organ_name: String,
    #[serde(rename = "attachmentFileName")]
    pub attachment_file_name: String,
    #[serde(rename = "attachmentFileKey")]
    pub attachment_file_key: String,
    #[serde(rename = "attachmentFileCntForCd1")]
    pub attachment_file_cnt_for_cd_1: String,
    #[serde(rename = "attachmentFileCntForCd2")]
    pub attachment_file_cnt_for_cd_2: String,
}

impl GitekiDevice {
    pub fn from_giteki_info(giteki_info: giteki::list::GitekiInfo) -> GitekiDevice {
        GitekiDevice {
            tech_code: giteki_info.tech_code,
            number: giteki_info.number,
            date: giteki_info.date,
            name: giteki_info.name,
            radio_equipment_code: giteki_info.radio_equipment_code,
            type_name: giteki_info.type_name,
            elec_wave: giteki_info.elec_wave,
            spurious_rules: giteki_info.spurious_rules,
            body_sar: giteki_info.body_sar,
            note: giteki_info.note,
            organ_name: giteki_info.organ_name,
            attachment_file_name: giteki_info.attachment_file_name,
            attachment_file_key: giteki_info.attachment_file_key,
            attachment_file_cnt_for_cd_1: giteki_info.attachment_file_cnt_for_cd_1,
            attachment_file_cnt_for_cd_2: giteki_info.attachment_file_cnt_for_cd_2,
        }
    }
}
