use swc_css_visit::Fold;

pub struct CSSRTLCompilerVisitor {}

impl CSSRTLCompilerVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Fold for CSSRTLCompilerVisitor {}
