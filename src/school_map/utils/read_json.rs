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

/// 將 json 檔案讀取為 Struct
pub fn read_json<T>(path_strings: &Vec<&str>) -> Result<T, io::Error>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    let content = read_file(path_strings)?;
    let data: T = serde_json::from_str(&content)?;
    Ok(data)
}
