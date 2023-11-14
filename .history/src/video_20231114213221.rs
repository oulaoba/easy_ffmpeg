pub enum EasyError {
    ChileError,
}

impl std::fmt::Display for EasyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!()
    }
}

impl std::error::Error for EasyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(Err(self))
    }
}

/// 视频画面剪切命令
pub fn ffmpeg_crop(
    file: String,
    width: u16,
    hight: u16,
    output_path: String,
    output_name: String,
) -> Result<String, EasyError> {
    let path = std::path::MAIN_SEPARATOR;
    let command = format!(
        " -i {} -vf crop=w={}:h={} {}{}{}",
        file, width, hight, output_path, path, output_name
    );
    Ok(command)
}

/// 视频画面缩放
pub fn ffmpeg_scale(
    file: String,
    width: u16,
    hight: u16,
    output_path: String,
    output_name: String,
) -> Result<String, EasyError> {
    let path = std::path::MAIN_SEPARATOR;
    let command = format!(
        "ffmpeg -i {} -vf scale=w='{}':h='{}' {}{}{}",
        file, width, hight, output_path, path, output_name
    );
    Ok(command)
}
