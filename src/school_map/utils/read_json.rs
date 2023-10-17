use std::fs::File;
use std::io::{self, Read};
use serde::{Serialize, Deserialize};

/// 讀取文件
fn read_file(path_strings: &Vec<&str>) -> Result<String, io::Error> {
    let mut filepath = std::env::current_dir().unwrap();
    filepath.extend(path_strings.iter());
    let filepath = format!("{}", filepath.display());
    let mut s = String::new();
    File::open(filepath)?.read_to_string(&mut s)?;
    Ok(s)
}

/// 讀取 JSON 檔案後轉成指定的資料型態
/// # 說明
/// - 輸入
///     - 檔案相對路徑
/// - 輸出
///     - Ok: T 對應到的資料型態
///     - Err: io::Error
/// - 泛型 T 必須實現兩種 Trait
///     - serde::Serialize
///     - serde::Deserialize
/// # Examples
/// ```
/// use crate::navigation_server::school_map::structs::NodesMap;
/// use crate::navigation_server::school_map::read_json;
/// 
/// let path = vec!["src", "assets", "data.json"]; //檔案路徑
/// let nodes = read_json::<NodesMap>(&path).unwrap(); //讀取檔案並轉為結構
/// ```
pub fn read_json<T>(path_strings: &Vec<&str>) -> Result<T, io::Error>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    let content = read_file(path_strings)?;
    let data: T = serde_json::from_str(&content)?;
    Ok(data)
}
