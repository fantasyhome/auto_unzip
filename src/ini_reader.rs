use std::env;
use std::path::PathBuf;

use ini::configparser::ini::Ini;

#[allow(dead_code)]
pub fn test_ini() {
    // 创建一个新的Ini对象
    let mut config = Ini::new();

    config
        .read(String::from(
            "[2000s]
       2020 = bad",
        ))
        .expect("read ini err");
    let mut exe_path = get_exe_path();
    exe_path = exe_path.join("config.ini");
    if let Some(path_str) = exe_path.to_str() {
        config.write(path_str).expect("write ini err");
    } else {
        println!("无法将 PathBuf 转换为 &str");
    }
}

pub struct IniInfo {
    pub input_dir: String,
    pub output_dir: String,
    pub password: String,
}

pub fn get_ini_info() -> IniInfo {
    let mut ini = Ini::new();
    let mut exe_path = get_exe_path();
    exe_path = exe_path.join("config.ini");
    if let Some(path_str) = exe_path.to_str() {
        ini.load(path_str).expect("ini file not found");
    } else {
        println!("无法将 PathBuf 转换为 &str");
    }
    IniInfo {
        input_dir: ini.get("common", "input_dir").expect("input_dir not found"),
        output_dir: ini
            .get("common", "output_dir")
            .expect("output_dir not found"),
        password: ini.get("common", "password").expect("password not found"),
    }
}

fn get_exe_path() -> PathBuf {
    // 获取当前程序的可执行文件路径
    let exe_path = env::current_exe().expect("无法获取当前可执行文件路径");

    // 获取所在目录
    let exe_dir = exe_path.parent().expect("无法获取所在目录");

    println!("程序所在的目录是: {:?}", exe_dir);
    exe_dir.to_path_buf()
}
