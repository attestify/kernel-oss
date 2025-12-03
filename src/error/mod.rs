#[cfg(test)] mod tests;

/// A structured error used across the kernel crates.
///
/// The `Error` type carries:
/// - `audience`: who should handle the error (user vs system),
/// - `kind`: a coarse category of error,
/// - `message`: a human-readable description stored as an owned `String`.
///
/// Purpose:
/// - Provide a lightweight, copyable error payload that can be matched on by callers,
/// - Be usable in collections (`Hash`, `Eq`) and for diagnostics (`Debug`, `Display`),
/// - Keep error handling simple: no backtrace or source chaining here, only classification + message.
///
/// Guarantees and design notes:
/// - `Error` derives `Clone`, `Debug`, `Eq`, `Hash`, and `PartialEq` so it can be cloned,
///   compared for equality, hashed into sets/maps, and debug-printed.
/// - Ordering (`Ord`/`PartialOrd`) is intentionally not relied on by callers; comparisons
///   should match on `audience`/`kind`/`message` explicitly when needed.
/// - `message` is an owned `String` so callers do not need to retain the originating input.
///
/// Public interfaces:
/// - `Error::new(audience, kind, message)`: construct any `Error`.
/// - `Error::for_user(kind, message)`: convenience constructor for user-facing errors.
/// - `Error::for_system(kind, message)`: convenience constructor for system-facing errors.
/// - `Error::is_user() -> bool` / `Error::is_system() -> bool`: quick audience checks.
/// - `Display` is implemented to format the `message` only (suitable for end-user display).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error {
    pub audience: Audience,
    pub kind: Kind,
    pub message: String,
}

impl Error {
    /// Create a new `Error` with the given audience, kind and message.
    ///
    /// `message` accepts any type that implements `Into<String>` for ergonomic callers.
    ///
    /// Example:
    /// ```rust
	/// use kernel_oss::error::Kind;
	/// use kernel_oss::error::Audience;
	/// use kernel_oss::error::Error;
	///
    /// let e = Error::new(Audience::User, Kind::InvalidInput, "missing field");
    /// ```
    pub fn new(audience: Audience, kind: Kind, message: impl Into<String>) -> Error {
        Error {
            audience,
            kind,
            message: message.into(),
        }
    }

    /// Convenience constructor for errors intended for end users.
    ///
    /// Sets `audience` to `Audience::User`.
    pub fn for_user(kind: Kind, message: impl Into<String>) -> Error {
        Error::new(Audience::User, kind, message)
    }

    /// Convenience constructor for errors intended for system/operational handling.
    ///
    /// Sets `audience` to `Audience::System`.
    pub fn for_system(kind: Kind, message: impl Into<String>) -> Error {
        Error::new(Audience::System, kind, message)
    }

    /// Returns `true` when the error is intended for a user-level audience.
    pub fn is_user(&self) -> bool {
        self.audience == Audience::User
    }

    /// Returns `true` when the error is intended for a system/operational audience.
    pub fn is_system(&self) -> bool {
        self.audience == Audience::System
    }
}

impl std::fmt::Display for Error {
    /// Displays only the human readable `message` field.
    ///
    /// This is intentional: `Display` is for short, user-facing text; use `Debug` to
    /// inspect struct fields during development.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}


/// Specifies who should handle an `Error`.
///
/// It is used with the [`Error`] type.
///
/// Handling guidance:
/// - Match on `Audience` to route handling logic; there are only two variants.
/// - `User` indicates the error is intended for end-user consumption (presentable text).
/// - `System` indicates the error is for internal/operational handling (logs, metrics).
/// - Do not rely on any ordering of variants; use equality or pattern matching as needed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Audience {
    User,
    System,
}

/// High-level categories for `Error` instances.
///
/// It is used with the [`Error`] type.
///
/// Guidance for callers:
/// - `Kind` is expected to grow; avoid exhaustive matches. Match the variants you care about
///   and use `_` to handle other/unknown cases.
/// - Treat `Kind` as a classification for routing or mapping to user messages/log levels.
/// - Do not rely on ordering of variants; use explicit matches or equality checks when needed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    /// A value exceeds the maximum allowed length.
    ExceedsMax,
    /// A value is below the minimum allowed length.
    BelowMin,
    /// An entity was not found.
    NotFound,
    /// The supplied input does not meet the required constraints.
    InvalidInput,
    /// Generally used as the default for a match statement when all other Kinds have been exhausted.
    Unexpected,
    /// A gateway execution error occurred when executing a gateway adapter implementation.
    GatewayError,
    /// A usecase execution error occurred when executing a usecase configuration.
    UsecaseError,
    /// A permission denied error occurred when a user does not have required permissions.
    PermissionDenied,
    /// When some type of local (in-memory) data processing failure occurs.
    ProcessingFailure,
}