//! Filters allow you to customize how strictly commands will be listened to.
//!
//! # Examples
//!
//! [`Filter::Strict`](crate::util::Filter::Strict) requires the command to match exactly:
//!
//! ```
//! use safe_vk::util::{matchit, Filter};
//!
//! let command = "/start";
//! let filter = &Filter::Strict;
//!
//! assert!(matchit("/start", command, filter)); // This would pass
//! assert!(!matchit("/START", command, filter)); // This would fail
//! assert!(!matchit(" /start", command, filter)); // This would fail
//! ```
//!
//! [`Filter::Flexible`](crate::util::Filter::Flexible) allows some variations like different cases and extra spaces:
//!
//! ```
//! use safe_vk::util::{matchit, Filter};
//!
//! let command = "/start";
//! let filter = &Filter::Flexible;
//!
//! assert!(matchit("/start", command, filter)); // This would pass
//! assert!(matchit("/START", command, filter)); // This would pass
//! assert!(matchit(" /start", command, filter)); // This would pass
//! ```
//!
//! [`Filter::Sensitive`](crate::util::Filter::Sensitive) will trigger without any symbol and is case-insensitive:
//!
//! ```
//! use safe_vk::util::{matchit, Filter};
//!
//! let command = "/start";
//! let filter = &Filter::Sensitive;
//!
//! assert!(matchit("start",  command, filter)); // This would pass
//! assert!(matchit("START",  command, filter)); // This would pass
//! assert!(matchit("!start", command, filter)); // This would pass
//! ```
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
    /// Must be exactly the same command
    Strict,
    /// Can have some spaces, and can be uppercase, but still must be the same command
    Flexible,
    /// Can be triggered without any symbol, and also can be uppercase
    Sensitive,
}

pub fn matchit(message: &str, command: &str, filter: &Filter) -> bool {
    let pattern = match filter {
        Filter::Strict => format!(r"^{}$", regex::escape(command)),
        Filter::Flexible => format!(r"(?i)^\s*[^\w\s]?{}\s*$", regex::escape(command)),
        Filter::Sensitive => format!(
            r"(?i)(?:^|[\W_]){}(?:[\W_]|$)",
            regex::escape(&command.trim_start_matches(|c: char| !c.is_alphanumeric()))
        ),
    };
    Regex::new(&pattern).unwrap().is_match(message)
}
