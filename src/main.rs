use std::path::Path;
use std::fs;
use std::process::Command;

#[tokio::main]
async fn main() {
    let input_folder_path = "D:\\일반폴더\\BackUp_IPhone\\1_heic";
    let output_folder_path = "D:\\일반폴더\\BackUp_IPhone\\converted_images4";

    if let Err(e) = fs::create_dir_all(output_folder_path) {
        eprintln!("출력 폴더 생성 실패: {:?}", e);
        return;
    }

    match fs::read_dir(input_folder_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("heic")) {
                    if let Some(input_file_path) = path.to_str() {
                        if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                            let output_file_path = Path::new(output_folder_path).join(format!("{}.jpg", file_stem));

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
