use image::{DynamicImage, ImageFormat};
use std::path::Path;
use std::fs::File;
use std::io::Write;

#[tauri::command]
fn write_file(path: String, data: Vec<u8>) -> Result<(), String> {
    println!("Writing file to: {}", path); // 添加日志输出
    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(&data).map_err(|e| e.to_string())?;
    println!("File written successfully"); // 添加日志输出
    Ok(())
}

#[tauri::command]
fn compress_image(path: String, quality: u8) -> Result<String, String> {
    let img = image::open(&Path::new(&path)).map_err(|e| e.to_string())?;
    
    // 转换为RGBA格式以保留透明通道
    let compressed = img.to_rgba8();

    // 获取源文件所在目录
    let parent_dir = Path::new(&path).parent()
        .ok_or("无法获取文件所在目录".to_string())?;
    
    // 获取源文件名（不含扩展名）
    let file_stem = Path::new(&path).file_stem()
        .ok_or("无法获取文件名".to_string())?
        .to_str()
        .ok_or("文件名转换失败".to_string())?;
    
    // 生成随机哈希值
    let random_hash = rand::random::<u64>().to_string();
    
    // 构建输出路径
    let output_path = parent_dir.join(format!("{}_{}_compressed.png", file_stem, random_hash));
    
    compressed.save_with_format(&output_path, ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(output_path.to_str().unwrap().to_string())
}

#[tauri::command]
fn get_file_path(file: String) -> String {
    // 这里可以根据需要返回文件的完整路径
    format!("/Users/lbz/Downloads/tauri-image/{}", file)
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![write_file, compress_image, get_file_path])  // Add get_file_path here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
