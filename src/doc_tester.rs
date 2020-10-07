use jsdoc::ast::{ExampleTag, Tag};
use jsdoc::Input;
use swc_common::{comments::SingleThreadedComments, Span};
use swc_ecmascript::visit::Visit;
use swc_ecmascript::{
    ast::{Class, Decl, ExportDecl, Expr, Function, Module, VarDecl},
    visit::Node,
};

struct DocTester {
    comments: SingleThreadedComments,
    examples: Vec<ExampleTag>,
}

// Currently Supports
// 1. comment on a class
// 2. comment on a class' method
// 3. comment on a function (including arrow functions)
// 4. comment var decl as far as its' init is an object, a function or a class
// 5. comment on an object method
// NB: user's duty not to run test on private functions

impl DocTester {
    fn new(comments: SingleThreadedComments) -> Self {
        Self {
            comments,
            examples: vec![],
        }
    }

    fn default() -> Self {
        Self {
            comments: SingleThreadedComments::default(),
            examples: vec![],
        }
    }

    fn visit(&mut self, module: &Module) {
        self.visit_module(module, module);
    }

    pub fn parse_span_comments(&mut self, span: Span) {
        let comments = self
            .comments
            .with_leading(span.lo, |comments| comments.to_vec());
        let examples = comments
            .iter()
            .map(|comment| {
                jsdoc::parse(Input::from(comment))
                    .expect("Unable to parse jsdoc")
                    .1
            })
            .flat_map(|js_doc| {
                js_doc
                    .tags
                    .into_iter()
                    .filter_map(|tag_item| match tag_item.tag {
                        Tag::Example(ex_tag) => Some(ex_tag),
                        _ => None,
                    })
            });
        self.examples.extend(examples);
    }

    pub fn check_var_decl(&mut self, var_decl: &VarDecl, opt_export_decl: Option<&ExportDecl>) {
        var_decl.decls.iter().for_each(|decl| {
            if let Some(expr) = &decl.init {
                match &**expr {
                    Expr::Object(_) | Expr::Fn(_) | Expr::Class(_) | Expr::Arrow(_) => {
                        if let Some(export_decl) = opt_export_decl {
                            self.parse_span_comments(export_decl.span);
                        } else {
                            self.parse_span_comments(var_decl.span);
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

impl Visit for DocTester {
    fn visit_class(&mut self, class: &Class, parent: &dyn Node) {
        self.parse_span_comments(class.span);
        swc_ecmascript::visit::visit_class(self, class, parent);
    }

    fn visit_function(&mut self, function: &Function, parent: &dyn Node) {
        self.parse_span_comments(function.span);
        swc_ecmascript::visit::visit_function(self, function, parent);
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl, parent: &dyn Node) {
        self.check_var_decl(var_decl, None);
        swc_ecmascript::visit::visit_var_decl(self, var_decl, parent);
    }

    fn visit_export_decl(&mut self, export_decl: &ExportDecl, parent: &dyn Node) {
        match &export_decl.decl {
            Decl::Var(var_decl) => self.check_var_decl(var_decl, Some(export_decl)),
            Decl::Class(_) | Decl::Fn(_) => self.parse_span_comments(export_decl.span),
            _ => {}
        }
        swc_ecmascript::visit::visit_export_decl(self, export_decl, parent);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::swc_util::AstParser;
    use swc_ecmascript::parser::Syntax;
    use swc_ecmascript::parser::TsConfig;
    #[test]
    fn test() {
        let source_code = std::fs::read_to_string("./js_test/test.ts").unwrap();
        let parser = AstParser::new();
        let (module_result, comments) = parser.parse_module(
            "test.ts",
            Syntax::Typescript(TsConfig::default()),
            &source_code,
        );
        let module = module_result.unwrap();
        let mut doctester = DocTester::new(comments);
        doctester.visit(&module);
        println!("{:#?}", doctester.examples);
    }
}
