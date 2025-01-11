use embedded_hal::blocking::i2c::WriteIter;

#[derive(Debug)]
pub enum MyHttpError {
    HttpRedirection(u16),
    HttpClientError(u16),
    HttpSeverError(u16)
}

impl std::fmt::Display for MyHttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyHttpError::HttpClientError(e) => write!(f, "客户端错误 {}", e),
            MyHttpError::HttpSeverError(e) => write!(f, "服务端错误 {}", e),
            MyHttpError::HttpRedirection(e) => write!(f, "错误 {}", e)
        }
    }
}

impl std::error::Error for MyHttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyHttpError::HttpClientError(e) => None,
            MyHttpError::HttpRedirection(e) => None,
            MyHttpError::HttpSeverError(e) => None,
        }
    }
}