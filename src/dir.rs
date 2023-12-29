use std::fs;
use std::io::Write;
use std::path::PathBuf;

use log::info;

use crate::consts::STORAGE_FILE_NAME;

pub fn init_data() {
    // Base data directory
    fs::create_dir_all(base_dir()).unwrap();

    // Recipe data
    let data_file = data_file();
    if fs::metadata(data_file.clone()).is_ok() {
        info!("数据文件已经存在")
    } else {
        // 文件不存在，创建新文件
        let mut file = fs::File::create(&data_file).unwrap();
        // 将空数组内容写入文件
        file.write_all("[]".as_bytes()).unwrap();
        info!("数据文件创建成功");
    }
}

pub fn base_dir() -> PathBuf {
    // PathBuf::from(format!("./data/{}", &*PEER_ID))
    PathBuf::from(format!("./data/{}", "test"))
}

pub fn data_file() -> PathBuf {
    base_dir().join(STORAGE_FILE_NAME)
}
