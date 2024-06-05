use std::mem;

use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_css_ast::{
    AttributeSelector, AttributeSelectorMatcher, AttributeSelectorMatcherValue,
    AttributeSelectorValue, Combinator, CombinatorValue, ComplexSelector, ComplexSelectorChildren,
    ComponentValue, CompoundSelector, Declaration, DeclarationName, FunctionName, Ident,
    NestingSelector, PseudoClassSelector, PseudoClassSelectorChildren, QualifiedRule,
    QualifiedRulePrelude, SelectorList, SimpleBlock, Str, SubclassSelector, Token, TokenAndSpan,
    TypeSelector, UniversalSelector, WqName,
};
use swc_css_visit::{VisitMut, VisitMutWith};

pub struct CSSRTLCompilerVisitor {}
impl VisitMut for CSSRTLCompilerVisitor {
    fn visit_mut_qualified_rule(&mut self, n: &mut QualifiedRule) {
        n.visit_mut_children_with(self);
        convert_block(&mut n.block);
    }

    fn visit_mut_at_rule(&mut self, n: &mut swc_css_ast::AtRule) {
        n.visit_mut_children_with(self);
        if let Some(ref mut block) = n.block {
            convert_block(block);
        }
    }
}

lazy_static! {
    // [dir="ltr"]
    static ref LTR_DIR_SELECTOR: ComplexSelector = ComplexSelector {
        span: DUMMY_SP,
        children: vec![
            ComplexSelectorChildren::CompoundSelector(
                CompoundSelector {
                    span: DUMMY_SP,
                    nesting_selector: None,
                    type_selector: None,
                    subclass_selectors: vec![
                        SubclassSelector::Attribute(Box::new(
                            AttributeSelector {
                                span: DUMMY_SP,
                                name: WqName {
                                    span: DUMMY_SP,
                                    prefix: None,
                                    value: Ident {
                                        span: DUMMY_SP,
                                        value: Atom::new("dir"),
                                        raw: Some(Atom::new(
                                            "dir"
                                        ))
                                    },
                                },
                                matcher: Some(AttributeSelectorMatcher{
                                    span: DUMMY_SP,
                                    value: AttributeSelectorMatcherValue::Equals,
                                }),
                                value: Some(AttributeSelectorValue::Str(Str {
                                    span: DUMMY_SP,
                                    value: Atom::new("ltr"),
                                    raw: None
                                })),
                                modifier: None
                            }
                        ))
                    ]
                }
            )
        ]
    };
    // [dir="rtl"]
    static ref RTL_DIR_SELECTOR: ComplexSelector = ComplexSelector {
        span: DUMMY_SP,
        children: vec![
            ComplexSelectorChildren::CompoundSelector(
                CompoundSelector {
                    span: DUMMY_SP,
                    nesting_selector: None,
                    type_selector: None,
                    subclass_selectors: vec![
                        SubclassSelector::Attribute(Box::new(
                            AttributeSelector {
                                span: DUMMY_SP,
                                name: WqName {
                                    span: DUMMY_SP,
                                    prefix: None,
                                    value: Ident {
                                        span: DUMMY_SP,
                                        value: Atom::new("dir"),
                                        raw: Some(Atom::new(
                                            "dir"
                                        ))
                                    },
                                },
                                matcher: Some(AttributeSelectorMatcher{
                                    span: DUMMY_SP,
                                    value: AttributeSelectorMatcherValue::Equals,
                                }),
                                value: Some(AttributeSelectorValue::Str(Str {
                                    span: DUMMY_SP,
                                    value: Atom::new("rtl"),
                                    raw: None
                                })),
                                modifier: None
                            }
                        ))
                    ]
                }
            )
        ]
    };

    // &:where([dir="ltr"], [dir="ltr"] *)
    static ref LTR_PRELUDE: QualifiedRulePrelude =
        QualifiedRulePrelude::SelectorList(SelectorList {
            span: DUMMY_SP,
            children: vec![ComplexSelector {
                span: DUMMY_SP,
                children: vec![ComplexSelectorChildren::CompoundSelector(
                    CompoundSelector {
                        span: DUMMY_SP,
                        nesting_selector: Some(NestingSelector { span: DUMMY_SP }),
                        type_selector: None,
                        subclass_selectors: vec![SubclassSelector::PseudoClass(
                            PseudoClassSelector {
                                span: DUMMY_SP,
                                name: Ident {
                                    span: DUMMY_SP,
                                    value: Atom::new("where"),
                                    raw: Some(Atom::new("where"))
                                },
                                children: Some(vec![PseudoClassSelectorChildren::SelectorList(
                                    SelectorList {
                                        span: DUMMY_SP,
                                        children: vec![
                                            // [dir="ltr"]
                                            LTR_DIR_SELECTOR.clone(),
                                            // [dir="ltr"] *
                                            {
                                                let mut selector = LTR_DIR_SELECTOR.clone();
                                                selector.children.push(
                                                    ComplexSelectorChildren::Combinator(Combinator {
                                                        span: DUMMY_SP,
                                                        value: CombinatorValue::Descendant
                                                    })
                                                );
                                                selector.children.push(
                                                    ComplexSelectorChildren::CompoundSelector(CompoundSelector{
                                                        span: DUMMY_SP,
                                                        nesting_selector: None,
                                                        type_selector: Some(Box::new(TypeSelector::Universal(UniversalSelector {span:DUMMY_SP, prefix: None }))),
                                                        subclass_selectors:vec![]
                                                    })
                                                );
                                                selector
                                            }
                                        ]
                                    }
                                )])
                            }
                        )]
                    }
                )],
            }],
        });

    // &:where([dir="rtl"], [dir="rtl"] *)
    static ref RTL_PRELUDE: QualifiedRulePrelude =
    QualifiedRulePrelude::SelectorList(SelectorList {
        span: DUMMY_SP,
        children: vec![ComplexSelector {
            span: DUMMY_SP,
            children: vec![ComplexSelectorChildren::CompoundSelector(
                CompoundSelector {
                    span: DUMMY_SP,
                    nesting_selector: Some(NestingSelector { span: DUMMY_SP }),
                    type_selector: None,
                    subclass_selectors: vec![SubclassSelector::PseudoClass(
                        PseudoClassSelector {
                            span: DUMMY_SP,
                            name: Ident {
                                span: DUMMY_SP,
                                value: Atom::new("where"),
                                raw: None
                            },
                            children: Some(vec![PseudoClassSelectorChildren::SelectorList(
                                SelectorList {
                                    span: DUMMY_SP,
                                    children: vec![
                                        // [dir="rtl"]
                                        RTL_DIR_SELECTOR.clone(),
                                        // [dir="rtl"] *
                                        {
                                            let mut selector = RTL_DIR_SELECTOR.clone();
                                            selector.children.push(
                                                ComplexSelectorChildren::Combinator(Combinator {
                                                    span: DUMMY_SP,
                                                    value: CombinatorValue::Descendant
                                                })
                                            );
                                            selector.children.push(
                                                ComplexSelectorChildren::CompoundSelector(CompoundSelector{
                                                    span: DUMMY_SP,
                                                    nesting_selector: None,
                                                    type_selector: Some(Box::new(TypeSelector::Universal(UniversalSelector {span:DUMMY_SP, prefix: None }))),
                                                    subclass_selectors:vec![]
                                                })
                                            );
                                            selector
                                        }
                                    ]
                                }
                            )])
                        }
                    )]
                }
            )],
        }],
    });
}

fn convert_declaration(
    decl: Declaration,
    ltr_vec: &mut Vec<ComponentValue>,
    rtl_vec: &mut Vec<ComponentValue>,
) -> Option<Declaration> {
    match &decl.name {
        DeclarationName::Ident(ref ident) => match (&ident.value).to_ascii_lowercase().as_ref() {
            str if str.contains("right") || str.contains("left") => {
                let rtl_name = ident
                    .value
                    .to_ascii_lowercase()
                    .as_ref()
                    .replace("left", "LEFT")
                    .replace("right", "left")
                    .replace("LEFT", "right");

                rtl_vec.push(ComponentValue::Declaration(Box::new(Declaration {
                    name: DeclarationName::Ident(Ident {
                        span: decl.span.clone(),
                        value: Atom::new(rtl_name),
                        raw: None,
                    }),
                    value: decl.value.clone(),
                    span: decl.span.clone(),
                    important: decl.important.clone(),
                })));
                ltr_vec.push(ComponentValue::Declaration(Box::new(decl)));
                None
            }
            "padding" | "margin" => {
                if decl.value.len() == 4 {
                    rtl_vec.push(ComponentValue::Declaration(Box::new(Declaration {
                        name: decl.name.clone(),
                        value: vec![
                            decl.value[0].clone(),
                            decl.value[3].clone(),
                            decl.value[2].clone(),
                            decl.value[1].clone(),
                        ],
                        span: decl.span.clone(),
                        important: decl.important.clone(),
                    })));
                    ltr_vec.push(ComponentValue::Declaration(Box::new(decl)));
                    None
                } else {
                    Some(decl)
                }
            }
            "direction" | "text-align" => {
                // Value visitor will mutate this
                rtl_vec.push(ComponentValue::Declaration(Box::new(decl.clone())));
                ltr_vec.push(ComponentValue::Declaration(Box::new(decl)));
                None
            }
            _ => Some(decl),
        },
        _ => Some(decl),
    }
}

fn convert_block(block: &mut SimpleBlock) {
    let mut rtl_block_visitor = CSSRTLCompilerRtlBlockVisitor {
        values_visitor: CSSRTLCompilerValuesVisitor {},
    };
    let mut commit = |new_value: &mut Vec<ComponentValue>,
                      ltr_vec: &mut Vec<ComponentValue>,
                      rtl_vec: &mut Vec<ComponentValue>| {
        if ltr_vec.len() > 0 {
            let ltr_rule = QualifiedRule {
                prelude: LTR_PRELUDE.clone(),
                block: SimpleBlock {
                    span: DUMMY_SP,
                    name: TokenAndSpan {
                        span: DUMMY_SP,
                        token: Token::LBrace,
                    },
                    value: mem::take(ltr_vec),
                },
                span: DUMMY_SP,
            };
            new_value.push(ComponentValue::QualifiedRule(Box::new(ltr_rule)));
        }
        if rtl_vec.len() > 0 {
            let mut rtl_rule = QualifiedRule {
                prelude: RTL_PRELUDE.clone(),
                block: SimpleBlock {
                    span: DUMMY_SP,
                    name: TokenAndSpan {
                        span: DUMMY_SP,
                        token: Token::LBrace,
                    },
                    value: mem::take(rtl_vec),
                },
                span: DUMMY_SP,
            };
            rtl_rule
                .block
                .visit_mut_children_with(&mut rtl_block_visitor);
            new_value.push(ComponentValue::QualifiedRule(Box::new(rtl_rule)));
            rtl_vec.clear();
        }
    };

    let old_value = std::mem::replace(&mut block.value, Vec::<ComponentValue>::new());
    let mut new_value = &mut block.value;

    let mut ltr_vec: Vec<ComponentValue> = vec![];
    let mut rtl_vec: Vec<ComponentValue> = vec![];

    for val in old_value {
        match val {
            ComponentValue::Declaration(decl) => {
                if let Some(decl) = convert_declaration(*decl, &mut ltr_vec, &mut rtl_vec) {
                    commit(new_value, &mut ltr_vec, &mut rtl_vec);
                    new_value.push(ComponentValue::Declaration(Box::new(decl)));
                }
            }
            _ => {
                commit(new_value, &mut ltr_vec, &mut rtl_vec);
                new_value.push(val)
            }
        }
    }

    commit(&mut new_value, &mut ltr_vec, &mut rtl_vec);
}

pub struct CSSRTLCompilerRtlBlockVisitor {
    pub values_visitor: CSSRTLCompilerValuesVisitor,
}
impl VisitMut for CSSRTLCompilerRtlBlockVisitor {
    fn visit_mut_declaration(&mut self, n: &mut Declaration) {
        n.value.visit_mut_with(&mut self.values_visitor);
    }
}

pub struct CSSRTLCompilerValuesVisitor {}
impl VisitMut for CSSRTLCompilerValuesVisitor {
    fn visit_mut_function(&mut self, n: &mut swc_css_ast::Function) {
        n.visit_mut_children_with(self);

        match &mut n.name {
            FunctionName::Ident(name) if name.value == "env" => match n.value.get_mut(0) {
                Some(ComponentValue::Ident(ident)) if ident.value == "safe-area-inset-left" => {
                    ident.value = "safe-area-inset-right".into();
                    ident.raw = Some("safe-area-inset-right".into());
                }
                Some(ComponentValue::Ident(ident)) if ident.value == "safe-area-inset-right" => {
                    ident.value = "safe-area-inset-left".into();
                    ident.raw = Some("safe-area-inset-left".into());
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        match n.value.as_ref() {
            "left" => {
                n.value = "right".into();
                n.raw = Some("right".into());
            }
            "right" => {
                n.value = "left".into();
                n.raw = Some("left".into());
            }
            "ltr" => {
                n.value = "rtl".into();
                n.raw = Some("rtl".into());
            }
            "rtl" => {
                n.value = "ltr".into();
                n.raw = Some("ltr".into());
            }
            _ => {}
        }
    }
}
