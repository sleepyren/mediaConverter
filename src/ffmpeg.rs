use std::path::Path;
use std::process::Command;

pub fn convert_file(input_path: &Path, selected_format: &str) -> Result<String, (String, String)> {
    let ext = input_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));

    let (target_ext, codec_args): (&str, &[&str]) = match selected_format {
        "MP4 (H.264)" => ("mp4", &["-c:v", "libx264"]),
        "PNG (lossless)" => ("png", &[]),
        "JPG (compressed)" => ("jpg", &[]),
        "WEBP (for web)" => ("webp", &[]),
        "AVIF (modern)" => ("avif", &["-c:v", "libaom-av1"]),
        _ => return Err(("Unsupported format selected.".to_string(), "Choose a valid format.".to_string())),
    };

    if ext == target_ext {
        return Err(("Already correct format.".to_string(), "No conversion needed.".to_string()));
    }

    let input_str = input_path.to_string_lossy().to_string();
    let output_path = parent.join(format!("{}_converted.{}", stem, target_ext));
    let output_str = output_path.to_string_lossy().to_string();

    let mut args = vec!["-y", "-i", &input_str];
    args.extend_from_slice(codec_args);
    args.push(&output_str);

    match Command::new("ffmpeg").args(&args).output() {
        Ok(o) if o.status.success() => Ok(output_str),
        Ok(o) => Err(("FFmpeg failed.".to_string(), String::from_utf8_lossy(&o.stderr).to_string())),
        Err(e) => Err(("Failed to run FFmpeg.".to_string(), e.to_string())),
    }
}