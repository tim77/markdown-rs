// https://github.com/Stebalien/horrorshow-rs
use horrorshow::Raw;
use horrorshow::helper::doctype;

// https://github.com/kivikakk/comrak
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Clone, Debug)]
pub struct Preview {
    comrak_options: ComrakOptions,
}

impl Preview {
    pub fn new() -> Preview {
        let comrak_options = ComrakOptions {
            hardbreaks: true,
            ext_table: true,
            ext_strikethrough: true,
            ..ComrakOptions::default()
        };

        Preview { comrak_options }
    }

    pub fn render(&self, markdown: &str) -> String {
        format!(
            "{}",
            html!(
                : doctype::HTML;
                html {
                    head {
                        script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js") {}
                        script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js") {}
                        script {
                            : Raw("hljs.initHighlightingOnLoad()")
                        }
                        style {
                            : ".hljs{display:block;overflow-x:auto;padding:0.5em;color:#333;background:#f8f8f8}.hljs-comment,.hljs-quote{color:#998;font-style:italic}.hljs-keyword,.hljs-selector-tag,.hljs-subst{color:#333;font-weight:bold}.hljs-number,.hljs-literal,.hljs-variable,.hljs-template-variable,.hljs-tag .hljs-attr{color:#008080}.hljs-string,.hljs-doctag{color:#d14}.hljs-title,.hljs-section,.hljs-selector-id{color:#900;font-weight:bold}.hljs-subst{font-weight:normal}.hljs-type,.hljs-class .hljs-title{color:#458;font-weight:bold}.hljs-tag,.hljs-name,.hljs-attribute{color:#000080;font-weight:normal}.hljs-regexp,.hljs-link{color:#009926}.hljs-symbol,.hljs-bullet{color:#990073}.hljs-built_in,.hljs-builtin-name{color:#0086b3}.hljs-meta{color:#999;font-weight:bold}.hljs-deletion{background:#fdd}.hljs-addition{background:#dfd}.hljs-emphasis{font-style:italic}.hljs-strong{font-weight:bold}";
                            : "body { width: 90%; margin: 0 auto; }";
                            : "img { max-width: 90% }"
                        }
                    }
                    body {
                        : Raw(markdown_to_html(markdown, &self.comrak_options));
                    }
                }
            )
        )
    }
}
