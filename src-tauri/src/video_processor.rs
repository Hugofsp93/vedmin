use std::process::Command;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    pub success: bool,
    pub message: String,
    pub output_path: Option<String>,
}

fn create_command(cmd_name: &str) -> Command {
    let mut cmd = Command::new(cmd_name);
    let path = std::env::var("PATH").unwrap_or_default();
    // macOS apps often lack Homebrew paths in their GUI environment
    cmd.env("PATH", format!("{}:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin", path));
    cmd
}

pub fn get_duration(file_path: &str) -> Result<f64, String> {
    let output = create_command("ffprobe")
        .args(&[
            "-v", "error",
            "-show_entries", "format=duration",
            "-of", "default=noprint_wrappers=1:nokey=1",
            file_path,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let duration_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    duration_str.parse::<f64>().map_err(|e| format!("Failed to parse duration: {}", e))
}


#[tauri::command]
pub async fn process_video_cmd(input_path: String, final_cut_sec: u8, output_dir: String) -> ProcessResult {
    // 1. Get duration
    let duration = match get_duration(&input_path) {
        Ok(d) => d,
        Err(e) => return ProcessResult { success: false, message: e, output_path: None },
    };

    // 2. Calculate new duration after trimming
    let trimmed_duration = duration - (final_cut_sec as f64);
    if trimmed_duration <= 0.0 {
        return ProcessResult {
            success: false,
            message: "Video is too short to be trimmed by that amount.".to_string(),
            output_path: None,
        };
    }

    // 3. Calculate speed factor to fit in 60 seconds
    let mut speed_factor = 1.0;
    if trimmed_duration > 60.0 {
        speed_factor = trimmed_duration / 60.0;
    }

    // 4. Construct the ffmpeg command.
    let final_out_dir = if output_dir.is_empty() {
        if let Ok(home) = std::env::var("HOME") {
            format!("{}/Desktop", home)
        } else {
            output_dir
        }
    } else {
        output_dir
    };

    let out_dir_path = Path::new(&final_out_dir);
    if !out_dir_path.exists() {
        if let Err(e) = std::fs::create_dir_all(out_dir_path) {
            return ProcessResult { success: false, message: format!("Failed to create output dir: {}", e), output_path: None };
        }
    }

    let output_file_name = "Hugo_APP.mp4".to_string();
    let final_output_path = out_dir_path.join(output_file_name);

    let trimmed_duration_str = trimmed_duration.to_string();
    let filter_complex = format!("[0:v]setpts=PTS/{speed_factor},fps=60[v]");
    let mut args = vec![
        "-y",
        "-t", &trimmed_duration_str,
        "-i", &input_path,
    ];

    if speed_factor == 1.0 {
        args.push("-vf");
        args.push("fps=60");
        args.push("-c:v");
        args.push("libx264");
        args.push("-preset");
        args.push("veryfast");
        args.push("-crf");
        args.push("28");
        args.push("-an"); // Força a remoção de qualquer áudio
    } else {
        args.push("-filter_complex");
        args.push(&filter_complex);
        args.push("-map");
        args.push("[v]");

        // Optimize for speed and file size
        args.push("-c:v");
        args.push("libx264");
        args.push("-preset");
        args.push("veryfast");
        args.push("-crf");
        args.push("28");
        args.push("-an"); // Força a remoção de qualquer áudio
    }

    // Force strict duration on the output file
    let target_duration_str = if trimmed_duration > 60.0 { "60.000".to_string() } else { format!("{:.3}", trimmed_duration) };

    // Limit the maximum writing time to exactly the calculated required length
    args.push("-t");
    args.push(&target_duration_str);

    args.push(final_output_path.to_str().unwrap());

    // Execute ffmpeg
    let output = create_command("ffmpeg")
        .args(&args)
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                ProcessResult {
                    success: true,
                    message: "Success".to_string(),
                    output_path: Some(final_output_path.to_string_lossy().to_string()),
                }
            } else {
                ProcessResult {
                    success: false,
                    message: String::from_utf8_lossy(&out.stderr).to_string(),
                    output_path: None,
                }
            }
        }
        Err(e) => {
            ProcessResult {
                success: false,
                message: format!("Failed to spawn ffmpeg: {}", e),
                output_path: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_duration() {
        let path = "/tmp/test_vedmin.mp4";
        if Path::new(path).exists() {
            let duration = get_duration(path).unwrap();
            assert!(duration >= 4.9 && duration <= 5.1, "Duration should be approx 5 seconds");
        }
    }

    #[tokio::test]
    async fn test_process_video_cmd() {
        let path = "/tmp/test_vedmin.mp4".to_string();
        if Path::new(&path).exists() {
            let out_dir = "/tmp/vedmin_out".to_string();
            let _ = fs::remove_dir_all(&out_dir);

            let res = process_video_cmd(path.clone(), 1, out_dir.clone()).await;
            assert!(res.success, "Processing failed: {}", res.message);

            let final_path = res.output_path.unwrap();
            let new_duration = get_duration(&final_path).unwrap();

            // Original is 5s, cut is 1s, new is 4s. Speed factor is 1 since 4s < 60s.
            assert!(new_duration >= 3.9 && new_duration <= 4.1, "New duration should be approx 4 seconds");
        }
    }
}
