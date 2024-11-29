use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = read_file("path/to/file")?;
    Ok(())
}

fn read_file(path: &str) -> Result<String> {
    // 使用context和with_context可以为错误添加更多信息
    std::fs::read_to_string(path).with_context(|| format!("Failed to read file at {}", path))
}
