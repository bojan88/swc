use super::*;
use crate::hygiene::Config;
use swc_ecma_parser::Syntax;
use swc_ecma_visit::{Fold, VisitMut, VisitMutWith};

struct TsHygiene {
    top_level_mark: Mark,
}

impl VisitMut for TsHygiene {
    fn visit_mut_ident(&mut self, i: &mut Ident) {
        if SyntaxContext::empty().apply_mark(self.top_level_mark) == i.span.ctxt {
            println!("ts_hygiene: {} is top-level", i.sym);
            return;
        }

        let ctxt = format!("{:?}", i.span.ctxt).replace("#", "");
        i.sym = format!("{}__{}", i.sym, ctxt).into();
        i.span = i.span.with_ctxt(SyntaxContext::empty());
    }

    fn visit_mut_member_expr(&mut self, n: &mut MemberExpr) {
        n.obj.visit_mut_with(self);
        if n.computed {
            n.prop.visit_mut_with(self);
        }
    }

    fn visit_mut_prop_name(&mut self, n: &mut PropName) {
        match n {
            PropName::Computed(n) => {
                n.visit_mut_with(self);
            }
            _ => {}
        }
    }

    fn visit_mut_ts_qualified_name(&mut self, q: &mut TsQualifiedName) {
        q.left.visit_mut_with(self);
    }
}

fn run_test_with_config<F, V>(
    syntax: Syntax,
    tr: F,
    src: &str,
    to: &str,
    config: crate::hygiene::Config,
) where
    F: FnOnce() -> V,
    V: Fold,
{
    crate::tests::test_transform(syntax, |_| tr(), src, to, true, config);
}

#[test]
fn test_mark_for() {
    ::testing::run_test(false, |_, _| {
        let mark1 = Mark::fresh(Mark::root());
        let mark2 = Mark::fresh(mark1);
        let mark3 = Mark::fresh(mark2);
        let mark4 = Mark::fresh(mark3);

        let folder1 = Resolver::new(Scope::new(ScopeKind::Block, mark1, None), true);
        let mut folder2 = Resolver::new(
            Scope::new(ScopeKind::Block, mark2, Some(&folder1.current)),
            true,
        );
        folder2.current.declared_symbols.insert("foo".into());

        let mut folder3 = Resolver::new(
            Scope::new(ScopeKind::Block, mark3, Some(&folder2.current)),
            true,
        );
        folder3.current.declared_symbols.insert("bar".into());
        assert_eq!(folder3.mark_for_ref(&"bar".into()), Some(mark3));

        let mut folder4 = Resolver::new(
            Scope::new(ScopeKind::Block, mark4, Some(&folder3.current)),
            true,
        );
        folder4.current.declared_symbols.insert("foo".into());

        assert_eq!(folder4.mark_for_ref(&"foo".into()), Some(mark4));
        assert_eq!(folder4.mark_for_ref(&"bar".into()), Some(mark3));
        Ok(())
    })
    .unwrap();
}

#[test]
fn issue_1279_1() {
    run_test_with_config(
        Default::default(),
        || resolver(),
        "class Foo {
            static f = 1;
            static g = Foo.f;
        }",
        "
        let Foo1 = class Foo {
            static f = 1;
            static g = Foo.f;
        };
        ",
        Config {
            keep_class_names: true,
        },
    );
}

#[test]
fn issue_1279_2() {
    run_test_with_config(
        Default::default(),
        || resolver(),
        "class Foo {
            static f = 1;
            static g = Foo.f;
            method() {
                class Foo {
                    static nested = 1;
                    static nested2 = Foo.nested;
                }
            }
        }",
        "
        let Foo1 = class Foo {
            static f = 1;
            static g = Foo.f;
            method() {
                let Foo2 = class Foo {
                    static nested = 1;
                    static nested2 = Foo.nested;
                };
            }
        };
        ",
        Config {
            keep_class_names: true,
        },
    );
}
