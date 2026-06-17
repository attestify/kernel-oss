//! Multi-line bounded text value.

use crate::values::Value;

/// Multi-line bounded text.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Block {
    /// Canonical block text.
    value: String,
}

impl Block {
    /// Create a new text block
    ///
    /// A Block is multi-line string which can contain an unbounded amount of text.
    /// When a new Line is created, it is sanitized by removing all whitespace from
    /// the start and end of the input. No other formatting characters are removed.
    ///
    /// # Arguments
    ///
    /// * `s`: an &str.
    ///
    /// returns: Line
    ///
    /// # Examples
    ///
    ///
    /// use crate::values::block::Block;
    ///
    /// let text_block = Block::new("Another block of text.\r\n\t");
    ///
    pub fn new(value: &str) -> Self {
        Block {
            value: value.to_string(),
        }
        .sanitize()
    }

    /// Create a new text block
    ///
    /// A Block is multi-line string which can contain an unbounded amount of text.
    /// When a new Line is created, it is sanitized by removing all whitespace from
    /// the start and end of the input. No other formatting characters are removed.
    ///
    /// # Arguments
    ///
    /// * `s`: a String.
    ///
    /// returns: Line
    ///
    /// # Examples
    ///
    ///
    /// use crate::values::block::Block;
    ///
    /// let text_block = Block::from_string(String::from("Another block of text.\r\n\t"));
    ///
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        Block { value: s.into() }.sanitize()
    }

    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Returns `true` when the block is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns the owned block text.
    pub fn value(&self) -> String {
        self.value.clone()
    }

    fn sanitize(&self) -> Self {
        Block {
            value: String::from(self.value.trim()),
        }
    }
}

impl Value for Block {
    type ValueType = str;

    fn value(&self) -> &Self::ValueType {
        self.value.as_str()
    }
}

// Implement PartialEq to compare Block to String.
impl PartialEq<String> for Block {
    fn eq(&self, other: &String) -> bool {
        self.value == *other
    }
}

// This allows for the reverse comparison: String to Block
impl PartialEq<Block> for String {
    fn eq(&self, other: &Block) -> bool {
        *self == other.value
    }
}

// Implement PartialEq to compare Block to &str.
impl PartialEq<&str> for Block {
    fn eq(&self, other: &&str) -> bool {
        self.value == *other
    }
}

// This allows for the reverse comparison: &str to Block
impl PartialEq<Block> for &str {
    fn eq(&self, other: &Block) -> bool {
        *self == other.value
    }
}

#[cfg(test)]
mod tests;
