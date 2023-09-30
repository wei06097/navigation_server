use super::super::types::NodesMap;
use std::fs::File;
use std::io::{self, Read};

/// 讀取文件
fn read_file(path_strings: &Vec<&str>) -> Result<String, io::Error> {
    let mut filepath = std::env::current_dir().unwrap();
    filepath.extend(path_strings.iter());
    let filepath = format!("{}", filepath.display());
    let mut s = String::new();
    File::open(filepath)?.read_to_string(&mut s)?;
    Ok(s)
}

/// 將 json 檔案讀取為 Node Struct
pub fn read_nodes(path_strings: &Vec<&str>) -> Result<NodesMap, io::Error> {
    let content = read_file(path_strings)?;
    let data: NodesMap = serde_json::from_str(&content)?;
    Ok(data)
}
