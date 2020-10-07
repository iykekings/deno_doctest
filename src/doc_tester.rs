use swc_common::{
    comments::{Comment, SingleThreadedComments},
    Span,
};
use swc_ecmascript::visit::Visit;
use swc_ecmascript::{
    ast::{
        Class, ClassMember, ClassMethod, Decl, ExportDecl, Expr, Function, MethodProp, Module,
        ObjectLit, Prop, PropOrSpread, VarDecl,
    },
    visit::Node,
};

struct DocTester {
    comments: SingleThreadedComments,
}

// struct DocTestVisitor {
//     comments: SingleThreadedComments,
// }

impl DocTester {
    fn new(comments: SingleThreadedComments) -> Self {
        Self { comments }
    }

    fn default() -> Self {
        Self {
            comments: SingleThreadedComments::default(),
        }
    }

    fn visit(&mut self, module: &Module) {
        self.visit_module(module, module);
    }
    pub fn get_span_comments(&self, span: Span) -> Vec<Comment> {
        self.comments
            .with_leading(span.lo, |comments| comments.to_vec())
    }
    pub fn check_object_lit(&self, object_literal: &ObjectLit) {
        object_literal
            .props
            .iter()
            .for_each(|prop_or_spread| match prop_or_spread {
                PropOrSpread::Prop(prop) => match &**prop {
                    Prop::Method(method_prop) => {
                        eprintln!("{:#?}", self.get_span_comments(method_prop.function.span));
                    }
                    _ => {}
                },
                _ => {}
            });
    }
    pub fn check_class(&self, class: &Class) {
        class
            .body
            .iter()
            .for_each(|class_member| match class_member {
                ClassMember::Constructor(constructor) => {
                    eprintln!("{:#?}", self.get_span_comments(constructor.span));
                }
                ClassMember::Method(class_method) => {
                    eprintln!("{:#?}", self.get_span_comments(class_method.span));
                }
                _ => {}
            })
    }

    pub fn check_var_decl(&self, var_decl: &VarDecl, export: Option<&ExportDecl>) {
        var_decl.decls.iter().for_each(|decl| {
            if let Some(expr) = &decl.init {
                match &**expr {
                    Expr::Object(object_lit) => {
                        if let Some(export_decl) = export {
                            eprintln!("{:#?}", self.get_span_comments(export_decl.span));
                        }
                        eprintln!("{:#?}", self.get_span_comments(var_decl.span));
                        self.check_object_lit(object_lit);
                    }
                    Expr::Class(class_expr) => {
                        if let Some(export_decl) = export {
                            eprintln!("{:#?}", self.get_span_comments(export_decl.span));
                        }
                        eprintln!("{:#?}", self.get_span_comments(var_decl.span));
                        self.check_class(&class_expr.class);
                    }
                    Expr::Fn(_) => {
                        if let Some(export_decl) = export {
                            eprintln!("{:#?}", self.get_span_comments(export_decl.span));
                        }
                        eprintln!("{:#?}", self.get_span_comments(var_decl.span));
                    }
                    _ => {}
                }
            }
        });
    }
}

impl Visit for DocTester {
    fn visit_class(&mut self, class: &Class, parent: &dyn Node) {
        eprintln!(
            "{}\n{:#?}",
            "VisitClass",
            self.get_span_comments(class.span)
        );
        swc_ecmascript::visit::visit_class(self, class, parent);
    }

    fn visit_function(&mut self, function: &Function, parent: &dyn Node) {
        eprintln!(
            "{}\n{:#?}",
            "VisitFunction",
            self.get_span_comments(function.span)
        );
        swc_ecmascript::visit::visit_function(self, function, parent);
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl, parent: &dyn Node) {
        eprintln!(
            "{}\n{:#?}",
            "VisitVarDecl",
            self.get_span_comments(var_decl.span)
        );
        swc_ecmascript::visit::visit_var_decl(self, var_decl, parent);
    }

    fn visit_export_decl(&mut self, export_decl: &ExportDecl, parent: &dyn Node) {
        match &export_decl.decl {
            Decl::Class(_) => {
                eprintln!(
                    "{}\n{:#?}",
                    "VisitClassExportDecl",
                    self.get_span_comments(export_decl.span)
                );
                // self.check_class(&class_decl.class);
            }
            Decl::Fn(_) => {
                eprintln!(
                    "{}\n{:#?}",
                    "VisitFunctionExportDecl",
                    self.get_span_comments(export_decl.span)
                );
            }
            Decl::Var(_) => {
                eprintln!(
                    "{}\n{:#?}",
                    "VisitVarExport",
                    self.get_span_comments(export_decl.span)
                );
            }
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
        // eprintln!("{:#?}", module);
        let mut doctester = DocTester::new(comments);
        doctester.visit(&module);
    }
}
