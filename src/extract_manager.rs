use crate::ini_reader::*;
use infer;
use std::fs;
use std::path::Path;
use std::process::Command;
use tanzhenhui_code_lib::file_helper;

pub struct ExtractManager {
    bz_dir: String,
    input_dir: String,
    output_dir: String,
    password: String,
}

impl ExtractManager {
    pub fn new() -> Self {
        let IniInfo {
            bz_dir,
            input_dir,
            output_dir,
            password,
        } = get_ini_info();
        ExtractManager {
            bz_dir,
            input_dir,
            output_dir,
            password,
        }
    }

    pub fn extract_videos_from_compressed_files(&self) {
        match fs::remove_dir_all(&self.output_dir) {
            Ok(_) => println!("All files in {} have been deleted.", &self.output_dir),
            Err(_e) => println!("no file in output need to be delete"),
        }

        // 创建输出目录，如果不存在
        if !Path::new(&self.output_dir).exists() {
            match fs::create_dir_all(&self.output_dir) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to create dir: {}", e),
            }
        }

        let input_path = Path::new(&self.input_dir);
        self.do_extract(input_path);
    }

    fn do_extract(&self, walking_directory: &Path) {
        self.clean_filename_in_dir(walking_directory);

        let mut last_filename = String::new();
        // 遍历输入目录的文件
        for entry in fs::read_dir(walking_directory).expect("failed read input_dir") {
            let entry = entry.expect("failed get entry");
            let entry_path = entry.path();

            // 如果是文件并且可能是压缩文件，尝试解压
            if entry_path.is_file() {
                let buffer = &fs::read(&entry_path).expect("failed to read buffer");
                if infer::is_archive(buffer) {
                    // 避免分卷压缩文件多次解压
                    let current_filename = file_helper::get_filename_before_dot(&entry_path);
                    if current_filename == last_filename {
                        continue;
                    }
                    last_filename = current_filename;
                    // 解压缩并递归提取
                    self.extract_selected_file(&entry_path);
                } else if infer::is_video(buffer) {
                    // 如果是视频文件，移动到输出目录
                    let file_name = entry_path.file_name().expect("failed to get file_name");
                    let output_file_path = Path::new(&self.output_dir).join(file_name);
                    fs::rename(&entry_path, &output_file_path).expect("failed to move video file");
                    println!("video file moved to: {}", output_file_path.display());
                }
            } else if entry_path.is_dir() {
                self.do_extract(&entry_path)
            }
        }
    }

    fn clean_filename_in_dir(&self, dir: &Path) {
        if !dir.is_dir() {
            return;
        }
        for entry in fs::read_dir(dir).expect("failed read dir") {
            let path = entry.expect("failed get entry").path();
            let path_str = path.to_str().expect("failed to_str");
            // 过滤掉非 ASCII 字符，只保留 ASCII 字符（删除中文）
            let cleaned_path_str: String = path_str.chars().filter(|c| c.is_ascii()).collect();
            let cleaned_path = Path::new(&cleaned_path_str);
            fs::rename(&path, &cleaned_path).expect("failed to clean path");
        }
    }

    fn extract_selected_file(&self, path: &Path) {
        // 创建临时解压目录
        let temp_dir_path = path
            .parent()
            .expect("no parent path")
            .join("temp_extracted");

        // 调用 bz.exe 进行解压缩
        let status = Command::new(&self.bz_dir)
            .args(&[
                "x",
                &format!("-o:{}", temp_dir_path.to_string_lossy()),
                &format!("-p:{}", &self.password),
                "-aou",
                path.to_string_lossy().as_ref(),
            ])
            .status()
            .expect("Failed to execute Bandizip");

        if status.success() {
            println!("Successfully extracted: {}", path.display());
            self.do_extract(&temp_dir_path);
        } else {
            eprintln!("Failed to extract: {}", path.display());
        }

        // 删除临时解压目录
        fs::remove_dir_all(&temp_dir_path).expect("failed to delete temp file");
    }
}

#[allow(dead_code)]
pub fn test_infer_crate() {
    let file_path = r"D:\BaiduNetdiskDownload\假视频1.z删除i";
    let file_path2 = r"D:\BaiduNetdiskDownload\捕获.PNG";
    assert!(infer::archive::is_zip(&fs::read(file_path).unwrap()));
    assert!(infer::is_archive(&fs::read(file_path).unwrap()));
    assert!(infer::image::is_png(&fs::read(file_path2).unwrap()));
}
