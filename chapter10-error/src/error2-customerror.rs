use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
use std::num::ParseIntError;
use std::str::Utf8Error;

fn main() -> Result<(), CustomError> {
    let path = "./dat";
    let v = read_file(path)?;
    let x = to_utf8(v.as_bytes())?;
    let u = to_u32(x)?;
    println!("num:{:?}", u);
    Ok(())
}

///读取文件内容
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

/// 转化为u32数字
fn to_u32(v: &str) -> Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}

#[derive(Debug)]
enum CustomError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}
impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            CustomError::IoError(ref e) => Some(e),
            CustomError::Utf8Error(ref e) => Some(e),
            CustomError::ParseIntError(ref e) => Some(e),
        }
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            CustomError::IoError(ref e) => e.fmt(f),
            CustomError::Utf8Error(ref e) => e.fmt(f),
            CustomError::ParseIntError(ref e) => e.fmt(f),
        }
    }
}

// 自定义error转换
impl From<ParseIntError> for CustomError {
    fn from(s: std::num::ParseIntError) -> Self {
        CustomError::ParseIntError(s)
    }
}

impl From<IoError> for CustomError {
    fn from(s: std::io::Error) -> Self {
        CustomError::IoError(s)
    }
}

impl From<Utf8Error> for CustomError {
    fn from(s: std::str::Utf8Error) -> Self {
        CustomError::Utf8Error(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num() -> std::result::Result<(), CustomError> {
        let path = "./dat";
        let v = read_file(path)?;
        let x = to_utf8(v.as_bytes())?;
        let u = to_u32(x)?;
        assert_eq!(u, 8);
        Ok(())
    }
}
