use crate::ini_reader::*;
use infer;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use tanzhenhui_code_lib::file_helper;

pub struct ExtractManager {
    bz_dir: String,
    input_dir: String,
    output_dir: String,
    password: String,
    get_video: bool,
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
            get_video: false,
        }
    }

    pub fn extract_videos_from_compressed_files(&mut self) {
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

        let input_dir = self.input_dir.clone();
        let input_path = Path::new(&input_dir);
        self.do_extract(input_path);
    }

    fn do_extract(&mut self, walking_directory: &Path) {
        file_helper::clean_filename_in_dir(walking_directory);

        let mut last_filename = String::new();
        // 遍历输入目录的文件
        match fs::read_dir(walking_directory) {
            Ok(entries) => {
                for entry in entries {
                    let entry = match entry {
                        Ok(e) => e,
                        Err(e) => {
                            eprintln!("Failed to get directory entry: {}", e);
                            continue;
                        }
                    };
                    let entry_path = entry.path();
                    println!("file: {}", entry_path.display());

                    // 如果是文件并且可能是压缩文件，尝试判断文件类型
                    if entry_path.is_file() {
                        let mut file = match File::open(&entry_path) {
                            Ok(f) => f,
                            Err(e) => {
                                eprintln!("Failed to open file: {}", e);
                                continue;
                            }
                        };

                        // 只读取前 16 个字节判断文件类型
                        let mut buffer = vec![0u8; 16];
                        match file.read(&mut buffer) {
                            Ok(n) => {
                                if n < 16 {
                                    eprintln!(
                                        "file: {},smaller than expected.",
                                        entry_path.display()
                                    );
                                    continue;
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read file: {}", e);
                                continue;
                            }
                        }

                        if infer::is_archive(&buffer) {
                            // 避免分卷压缩文件多次解压
                            if let Some((current_filename, _extension)) =
                                file_helper::split_filename_by_first_dot(&entry_path)
                            {
                                if current_filename == last_filename {
                                    continue;
                                }
                                last_filename = current_filename;
                                // 解压缩并递归提取
                                self.extract_selected_file(&entry_path);
                            } else {
                                println!("Invalid file name or no dot found");
                            }
                        } else if infer::is_video(&buffer) {
                            // 如果是视频文件，移动到输出目录
                            if let Some(file_name) = entry_path.file_name() {
                                let output_file_path = Path::new(&self.output_dir).join(file_name);
                                if let Err(e) = fs::rename(&entry_path, &output_file_path) {
                                    eprintln!("Failed to move video file: {}", e);
                                } else {
                                    self.get_video = true;
                                    println!("Video file moved to: {}", output_file_path.display());
                                }
                            } else {
                                eprintln!("Failed to get file name for video file");
                            }
                        }
                    } else if entry_path.is_dir() {
                        self.do_extract(&entry_path)
                    }
                }
            }
            Err(e) => eprintln!("Failed to read input directory: {}", e),
        }
    }

    fn extract_selected_file(&mut self, path: &Path) {
        println!("Start to extract: {}", path.display());
        // 创建临时解压目录
        let temp_dir_path = match path.parent() {
            Some(parent) => parent.join("temp_extracted"),
            None => {
                eprintln!("no parent path for {}", path.display());
                return; // 如果没有父目录，直接返回
            }
        };

        // 调用 bz.exe 进行解压缩
        let status = match Command::new(&self.bz_dir)
            .args(&[
                "x",
                &format!("-o:{}", temp_dir_path.to_string_lossy()),
                &format!("-p:{}", &self.password),
                "-aou",
                path.to_string_lossy().as_ref(),
            ])
            .status()
        {
            Ok(status) => status,
            Err(e) => {
                eprintln!("Failed to execute Bandizip for {}: {}", path.display(), e);
                return; // 如果执行失败，直接返回
            }
        };

        if status.success() {
            println!("Successfully extracted: {}", path.display());
            self.do_extract(&temp_dir_path);
        } else {
            eprintln!("Failed to extract: {}", path.display());
        }

        // 删除临时解压目录
        if self.get_video {
            if let Err(e) = fs::remove_dir_all(&temp_dir_path) {
                eprintln!(
                    "Failed to delete temp dir {}: {}",
                    temp_dir_path.display(),
                    e
                );
            }
        }
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
