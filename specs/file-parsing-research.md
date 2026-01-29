# File Parsing Research (Rust)

## Scope
Text extraction for PDF, DOCX, and TXT uploads in the Rust backend.

## Decision Summary
- **PDF:** Use `pdf_extract` for primary text extraction.
- **DOCX:** Parse DOCX using `zip` + `quick-xml` (read `word/document.xml`).
- **TXT:** Read as bytes and decode with `chardetng` + `encoding_rs`, defaulting to UTF-8.

## PDF
### Decision
Use `pdf_extract` for extracting text from PDFs.

### Rationale
- Simple API focused on text extraction.
- Pure Rust (no native system dependencies), easier deployment on macOS/Linux.
- Good enough quality for most academic PDFs and scanned text (when text layer exists).

### Alternatives
- **`lopdf`**: Lower-level; more control over structure but more code to handle layout.
- **`pdfium-render`**: Higher quality extraction but requires bundling or installing PDFium.
- **`poppler`/`poppler-rs`**: Requires native Poppler libraries; more ops complexity.

## DOCX
### Decision
Implement DOCX text extraction by reading the ZIP container and parsing `word/document.xml` with `quick-xml`.

### Rationale
- DOCX is a ZIP package; the text resides primarily in `word/document.xml`.
- `quick-xml` is fast and stable for streaming XML parsing.
- Keeps dependencies small and avoids crates that focus on DOCX generation rather than reading.

### Alternatives
- **`docx` crate**: Convenience parsing but may be incomplete for complex docs.
- **`docx-rs`**: Primarily for generating DOCX, not reliable for extraction.
- **External converters (pandoc/libreoffice)**: Better fidelity but heavy runtime dependencies.

## TXT
### Decision
Read file bytes and detect encoding with `chardetng`, then decode with `encoding_rs`. Fall back to UTF-8 if detection is uncertain.

### Rationale
- Real-world TXT uploads often contain non-UTF-8 encodings.
- `encoding_rs` is the standard, fast decoder used by browsers.

### Alternatives
- **Assume UTF-8 only**: Simpler but fails on Windows-1252/GBK/etc.
- **`charset-normalizer`**: Not available in Rust; heavier solutions require external tools.

## Notes & Implementation Considerations
- Extraction should be streaming and chunked to avoid loading huge files into memory.
- Normalize whitespace and collapse excessive newlines for model input.
- Consider page separators for PDF to preserve context.
- For DOCX, skip header/footer unless explicitly needed.
