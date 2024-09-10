mod ini_reader;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::ini_reader::*;

fn main() -> io::Result<()> {
    let IniInfo {
        input_dir,
        output_dir,
        password,
    } = get_ini_info();

    // test_ini();
    clear_output();

    // 调用函数查找并提取视频文件
    extract_videos_from_compressed_files(&*input_dir, &*output_dir, &*password)?;

    Ok(())
}

fn delete_all_files_in_dir(dir: &str) -> io::Result<()> {
    // 将目录路径转换为Path对象
    let path = Path::new(dir);

    // 检查目录是否存在
    if path.exists() && path.is_dir() {
        // 遍历目录下的文件和文件夹
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            // 删除文件或文件夹
            if entry_path.is_file() {
                fs::remove_file(entry_path)?;
            } else if entry_path.is_dir() {
                fs::remove_dir_all(entry_path)?;
            }
        }
    } else {
        println!("Directory does not exist or is not a directory.");
    }

    Ok(())
}

fn clear_output() {
    let output_dir = r"D:\test\output";

    // 调用删除函数，并处理返回的错误
    match delete_all_files_in_dir(output_dir) {
        Ok(_) => println!("All files in {} have been deleted.", output_dir),
        Err(e) => eprintln!("Failed to delete files: {}", e),
    }
}

fn extract_videos_from_compressed_files(
    input_dir: &str,
    output_dir: &str,
    password: &str,
) -> io::Result<()> {
    let input_path = Path::new(input_dir);

    // 创建输出目录，如果不存在
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    // 遍历输入目录的文件
    for entry in fs::read_dir(input_path)? {
        let entry = entry?;
        let entry_path = entry.path();

        // 如果是文件并且可能是压缩文件，尝试解压
        if entry_path.is_file() {
            if is_compressed_file(&entry_path) {
                // 解压缩并递归提取
                extract_compressed_file(&entry_path, PathBuf::from(output_dir), password)?;
            }
        }
    }

    Ok(())
}

// 判断文件是否是可能的压缩文件（包括非常规后缀名）
fn is_compressed_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        // 将扩展名转换为字符串，处理可能的非UTF-8字符（不会删除中文）
        let ext_lossy = ext.to_string_lossy();

        // 过滤掉非 ASCII 字符，只保留 ASCII 字符（删除中文）
        let mut cleaned_ext: String = ext_lossy.chars().filter(|c| c.is_ascii()).collect();
        cleaned_ext = cleaned_ext.to_lowercase();
        let compress_extension_flag = [
            "zip","rar","7z","tar","gz","7","zi"
        ];
        for flag in compress_extension_flag
        {
            if cleaned_ext.eq(flag)
            {
                return true;
            }
        }
    }
    false
}

// 解压缩文件并提取其中的视频文件
fn extract_compressed_file(
    file_path: &Path,
    output_dir: PathBuf,
    password: &str,
) -> io::Result<()> {
    let file_name = file_path.file_name().unwrap().to_string_lossy();

    // 创建临时解压目录
    let temp_dir = format!("{}_extracted", file_name);
    let temp_dir_path = output_dir.join(temp_dir);

    // 调用 bz.exe 进行解压缩
    let bz_path = r"D:\tool\Bandizip\bz.exe"; // 修改为你的 bz.exe 路径
    let status = Command::new(bz_path)
        .args(&[
            "x",
            &format!("-o:{}", temp_dir_path.to_string_lossy()),
            &format!("-p:{}", password),
            "-aou",
            file_path.to_string_lossy().as_ref(),
        ])
        .status()
        .expect("Failed to execute Bandizip");

    if status.success() {
        println!("Successfully extracted: {}", file_path.display());

        // 递归检查解压出来的文件，提取视频文件或进一步解压
        for entry in fs::read_dir(&temp_dir_path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                if is_video_file(&entry_path) {
                    // 如果是视频文件，移动到输出目录
                    let file_name = entry_path.file_name().unwrap();
                    let output_path = Path::new(r"D:\test\output").join(file_name);
                    fs::rename(&entry_path, output_path)?;
                    println!("Extracted video file: {}", entry_path.display());
                } else if is_compressed_file(&entry_path) {
                    // 如果是压缩文件，递归解压
                    extract_compressed_file(&entry_path, temp_dir_path.clone(), password)?;
                }
            } else if entry_path.is_dir() {
                // 遍历输入目录的文件
                for entry in fs::read_dir(entry_path)? {
                    let entry = entry?;
                    let path = entry.path();

                    // 如果是文件并且可能是压缩文件，尝试解压
                    if path.is_file() {
                        if is_compressed_file(&path) {
                            // 解压缩并递归提取
                            extract_compressed_file(&path, output_dir.clone(), password)?;
                        } else if is_video_file(&path) {
                            // 如果是视频文件，移动到输出目录
                            let file_name = path.file_name().unwrap();
                            let output_path = Path::new(r"D:\test\output").join(file_name);
                            fs::rename(&path, output_path)?;
                            println!("Extracted video file: {}", path.display());
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to extract: {}", file_path.display());
    }

    // 删除临时解压目录
    fs::remove_dir_all(&temp_dir_path)?;

    Ok(())
}

// 判断文件是否是视频文件
fn is_video_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        return ext == "mp4" || ext == "avi" || ext == "mkv" || ext == "mov" || ext == "flv";
        // 常见的视频文件格式
    }
    false
}
