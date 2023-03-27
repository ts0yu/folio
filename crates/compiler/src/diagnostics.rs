use codemap::CodeMap;
use codemap_diagnostic::{ Level, SpanLabel, SpanStyle, Diagnostic, ColorConfig, Emitter };

pub struct Diagnostics;

impl Diagnostics {
    pub fn emit(code: String, file_name: String, start: u64, end: u64, label: String, message: String, error_code: String) {
        let mut codemap = CodeMap::new();
        let file_span = codemap.add_file(file_name, code).span;
        let name_span = file_span.subspan(start, end);
      
        let label = SpanLabel {
            span: name_span,
            style: SpanStyle::Primary,
            label: Some(label)
        };
    
        let d = Diagnostic {
            level: Level::Error,
            message,
            code: Some(error_code),
            spans: vec![label]
        };
      
        let mut emitter = Emitter::stderr(ColorConfig::Always, Some(&codemap));
        emitter.emit(&[d]);
    }
}