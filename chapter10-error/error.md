# error

![img.png](error-situation.png)

Rust 偷师 Haskell，构建了对标 Maybe 的 Option 类型和 对标 Either 的 Result 类型。

![img.png](option_n_result.png)

## ? 操作符

早期 Rust 提供了 try! 宏来简化错误的显式处理，后来为了进一步提升用户体验，try! 被进化成 ? 操作符

如果你只想传播错误，不想就地处理，可以用 ? 操作符

## Error trait 和错误类型的转换

```rust
// intellij-rust/stdlib-local-copy/1.78.0-ba6645f9113d6a36b36e041b03064c99e1ae9e85/library/core/src/error.rs
pub trait Error: Debug + Display {
    #[stable(feature = "error_source", since = "1.30.0")]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    /// Gets the `TypeId` of `self`.
    #[doc(hidden)]
    #[unstable(
        feature = "error_type_id",
        reason = "this is memory-unsafe to override in user code",
        issue = "60784"
    )]
    fn type_id(&self, _: private::Internal) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[deprecated(since = "1.42.0", note = "use the Display impl or to_string()")]
    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    #[stable(feature = "rust1", since = "1.0.0")]
    #[deprecated(
        since = "1.33.0",
        note = "replaced by Error::source, which can support downcasting"
    )]
    #[allow(missing_docs)]
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    /// ```
    #[unstable(feature = "error_generic_member_access", issue = "99301")]
    #[allow(unused_variables)]
    fn provide<'a>(&'a self, request: &mut Request<'a>) {}
}
```

## thiserror

```rust
use thiserror::Error;
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

不使用 thiserror

```rust
use std::fmt;

#[derive(Debug)]
enum DataFetchError {
    HttpError(u16),
    Timeout,
    InvalidPayload,
}

impl fmt::Display for DataFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpError(code) => write!(f, "HTTP error with code: {}", code),
            Self::Timeout => write!(f, "Data fetching timed out"),
            Self::InvalidPayload => write!(f, "Invalid payload received"),
        }
    }
}

impl std::error::Error for DataFetchError {}

#[derive(Debug)]
enum DatabaseError {
    ConnectionFailed,
    WriteFailed(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed => write!(f, "Failed to connect to database"),
            Self::WriteFailed(reason) => write!(f, "Failed to write to database: {}", reason),
        }
    }
}

impl std::error::Error for DatabaseError {}
```

使用 thiserror

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum DataFetchError {
    #[error("HTTP error with code: {0}")]
    HttpError(u16),
    #[error("Data fetching timed out")]
    Timeout,
    #[error("Invalid payload received")]
    InvalidPayload,
}

#[derive(Debug, Error)]
enum DatabaseError {
    #[error("Failed to connect to database")]
    ConnectionFailed,
    #[error("Failed to write to database: {0}")]
    WriteFailed(String),
}

```

- 代码减少: 对于每种错误类型，我们都不再需要单独的Display和Error trait实现。这大大减少了样板代码，并提高了代码的可读性。
- 错误消息与定义在一起: 使用thiserror，我们可以直接在错误定义旁边写出错误消息。这使得代码更加组织化，方便查找和修改。
- 可维护性增加: 如果我们要添加或删除错误类型，只需要修改枚举定义并更新错误消息即可，而不需要在其他地方进行更改

