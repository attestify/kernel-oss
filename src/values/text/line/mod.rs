//! Single-line bounded text value.

use crate::values::Value;

/// Single-line bounded text.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Line {
    /// Canonical line text.
    value: String,
}

impl Line {
    /// Create a text line from a &str
    ///
    /// A Line is single line string which can contain an unbounded amount of text.
    /// When a new Line is created, it is sanitized by removing: Carriage Returns ('\r'),
    /// Line feeds ('\n'), Tabs ('\t'), any whitespace that has more than one consecutive space,
    /// and all whitespace from the start and end of the input.
    ///
    /// # Arguments
    ///
    /// * `s`: an &str
    ///
    /// returns: Line
    ///
    /// # Examples
    ///
    ///
    /// use crate::values::line::Line;
    ///
    /// let text_line = Line::new("Another line of text.");
    ///
    pub fn new(line: &str) -> Self {
        Line {
            value: line.to_string(),
        }
        .sanitize()
    }

    /// Create a text line from a String
    ///
    /// A Line is single line string which can contain an unbounded amount of text.
    /// When a new Line is created, it is sanitized by removing: Carriage Returns ('\r'),
    /// Line feeds ('\n'), Tabs ('\t'), any whitespace that has more than one consecutive space,
    /// and all whitespace from the start and end of the input.
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
    /// use crate::values::line::Line;
    ///
    /// let text_line = Line::from_string(String::from("Another line of text."));
    ///
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        Line { value: s.into() }.sanitize()
    }

    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Returns `true` when the line is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns the owned line text.
    pub fn value(&self) -> String {
        self.value.clone()
    }

    fn sanitize(&self) -> Self {
        Line {
            value: self
                .value
                .replace("\n", "")
                .replace("\r", "")
                .replace("\t", "")
                .trim()
                .to_string(),
        }
    }
}

impl Value for Line {
    type ValueType = str;

    fn value(&self) -> &Self::ValueType {
        self.value.as_str()
    }
}

// Implement PartialEq to compare Line to String.
impl PartialEq<String> for Line {
    fn eq(&self, other: &String) -> bool {
        self.value == *other
    }
}

// This allows for the reverse comparison: String to Line
impl PartialEq<Line> for String {
    fn eq(&self, other: &Line) -> bool {
        *self == other.value
    }
}

// Implement PartialEq to compare Line to &str.
impl PartialEq<&str> for Line {
    fn eq(&self, other: &&str) -> bool {
        self.value == *other
    }
}

// This allows for the reverse comparison: &str to Line
impl PartialEq<Line> for &str {
    fn eq(&self, other: &Line) -> bool {
        *self == other.value
    }
}

#[cfg(test)]
mod tests;
