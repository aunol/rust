use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use rfd::FileDialog;

#[tokio::main]
async fn main() {
    // 폴더 선택 창 띄우기
    let input_folder_path = match FileDialog::new().pick_folder() {
        Some(path) => path,
        None => {
            eprintln!("폴더가 선택되지 않았습니다.");
            return;
        }
    };

    // 출력 폴더 설정 (선택된 폴더 내에 'converted_images' 폴더 생성)
    let output_folder_path = input_folder_path.join("converted_images");

    if let Err(e) = fs::create_dir_all(&output_folder_path) {
        eprintln!("출력 폴더 생성 실패: {:?}", e);
        return;
    }

    match fs::read_dir(&input_folder_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("heic")) {
                    if let Some(input_file_path) = path.to_str() {
                        if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                            let output_file_path = output_folder_path.join(format!("{}.jpg", file_stem));

                            match Command::new("ffmpeg")
                                .arg("-i")
                                .arg(input_file_path)
                                .arg("-vf")
                                .arg("format=yuv420p")
                                .arg("-pix_fmt")
                                .arg("yuvj420p")
                                .arg(output_file_path.to_str().unwrap())
                                .output() {
                                Ok(output) if output.status.success() => {
                                    println!("{} 변환 성공!", input_file_path);
                                }
                                Ok(output) => {
                                    eprintln!("{} 변환 실패: STDERR: {}", input_file_path, String::from_utf8_lossy(&output.stderr));
                                }
                                Err(e) => {
                                    eprintln!("{} 변환 실패: {:?}", input_file_path, e);
                                }
                            }
                        }
                    }
                }
            }
            println!("모든 변환 완료!");
        }
        Err(e) => eprintln!("입력 폴더 읽기 실패: {:?}", e),
    }
}
