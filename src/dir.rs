use std::fs;
use std::io::Write;
use std::path::PathBuf;

use log::info;

use crate::consts::{PEER_ID, STORAGE_FILE_NAME, SYNC_LOG_FILE_NAME};

pub fn init_data() {
    // Create data directory
    fs::create_dir_all(base_dir()).unwrap();
    fs::create_dir_all(op_log_dir()).unwrap();
    fs::create_dir_all(sync_log_dir()).unwrap();

    // Recipe data
    let data_file = data_file();
    if fs::metadata(data_file.clone()).is_ok() {
        info!("数据文件已经存在")
    } else {
        // 文件不存在，创建新文件
        let mut file = fs::File::create(&data_file).unwrap();
        // 将空数组内容写入文件
        file.write_all("{}".as_bytes()).unwrap();
        info!("数据文件创建成功");
    }
}

pub fn base_dir() -> PathBuf {
    PathBuf::from(format!("./data/{}", &*PEER_ID))
    // PathBuf::from(format!("./data/{}", "test"))
}

pub fn op_log_dir() -> PathBuf {
    base_dir().join("op_log")
}

pub fn sync_log_dir() -> PathBuf {
    base_dir().join("sync_log")
}

pub fn data_file() -> PathBuf {
    base_dir().join(STORAGE_FILE_NAME)
}

pub fn sync_log_file() -> PathBuf {
    sync_log_dir().join(SYNC_LOG_FILE_NAME)
}

// create r name|recipe_ingredients|recipe_instruction