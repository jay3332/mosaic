//! Defines the `SourceCache` type for storing source text.

use crate::span::{SourceId, Span};

/// A single source file stored in a [`SourceCache`].
struct SourceFile {
    name: String,
    source: String,
    /// Byte offset of the start of each line.
    line_starts: Vec<usize>,
}

impl SourceFile {
    fn new(name: String, source: String) -> Self {
        let line_starts = compute_line_starts(&source);
        Self {
            name,
            source,
            line_starts,
        }
    }
}

/// A line and column position (1-based).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineCol {
    /// Line number, starting from 1
    pub line: usize,
    /// Column number, starting from 1, in bytes
    pub col: usize,
}

/// Stores the lookup cache for source files.
pub struct SourceCache {
    /// The collection of source files.
    files: Vec<SourceFile>,
}

impl SourceCache {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Adds a source file to the cache and returns its [`SourceId`].
    pub fn add(&mut self, name: String, source: String) -> SourceId {
        let id = SourceId(self.files.len());
        self.files.push(SourceFile::new(name, source));
        id
    }

    /// Returns the filename for the source file associated with the given [`SourceId`].
    pub fn name(&self, id: SourceId) -> &str {
        &self.files[id.0].name
    }

    /// Returns the full source text for the source file associated with the given [`SourceId`].
    pub fn source(&self, id: SourceId) -> &str {
        &self.files[id.0].source
    }

    /// Returns the text covered by a span.
    pub fn span_text(&self, span: Span) -> &str {
        let source = self.source(span.source);
        &source[span.start..span.end]
    }

    /// Converts a byte offset within a file to a [`LineCol`].
    pub fn line_col(&self, id: SourceId, offset: usize) -> LineCol {
        let file = &self.files[id.0];
        let line_starts = &file.line_starts;

        let line_idx = match line_starts.binary_search(&offset) {
            Ok(exact) => exact,
            Err(next) => next - 1,
        };

        LineCol {
            line: line_idx + 1,
            col: offset - line_starts[line_idx] + 1,
        }
    }

    /// Returns the 1-based line and column for the start of a span
    pub fn span_start(&self, span: Span) -> LineCol {
        self.line_col(span.source, span.start)
    }

    /// Returns the full text of the line containing a given byte offset
    pub fn source_line(&self, id: SourceId, offset: usize) -> &str {
        let file = &self.files[id.0];
        let line_starts = &file.line_starts;

        let line_idx = match line_starts.binary_search(&offset) {
            Ok(exact) => exact,
            Err(next) => next - 1,
        };

        let start = line_starts[line_idx];
        let end = line_starts
            .get(line_idx + 1)
            .copied()
            .unwrap_or(file.source.len());

        // trim trailing newline (i.e. preceding trailing newlines, consider it eof)
        file.source[start..end].trim_end_matches(['\n', '\r'])
    }

    /// Returns the number of lines in the source file associated with the given [`SourceId`].
    pub fn line_count(&self, id: SourceId) -> usize {
        self.files[id.0].line_starts.len()
    }
}

impl Default for SourceCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the byte offset of the start of each line.
fn compute_line_starts(source: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (i, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_basic() {
        let mut cache = SourceCache::new();
        let id = cache.add("test.mosaic".to_string(), "hello\nworld\n!".to_string());

        assert_eq!(cache.line_col(id, 0), LineCol { line: 1, col: 1 });
        assert_eq!(cache.line_col(id, 5), LineCol { line: 1, col: 6 }); // '\n'
        assert_eq!(cache.line_col(id, 6), LineCol { line: 2, col: 1 }); // 'w'
        assert_eq!(cache.line_col(id, 12), LineCol { line: 3, col: 1 }); // '!'
    }

    #[test]
    fn source_line_basic() {
        let mut cache = SourceCache::new();
        let id = cache.add(
            "test.mosaic".to_string(),
            "first\nsecond\nthird".to_string(),
        );

        assert_eq!(cache.source_line(id, 0), "first");
        assert_eq!(cache.source_line(id, 6), "second");
        assert_eq!(cache.source_line(id, 13), "third");
    }

    #[test]
    fn span_text() {
        let mut cache = SourceCache::new();
        let id = cache.add("test.mosaic".to_string(), "hello world".to_string());
        let span = Span::new(id, 6, 11);
        assert_eq!(cache.span_text(span), "world");
    }
}
