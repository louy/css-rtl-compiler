use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_css_ast::{
    AttributeSelector, AttributeSelectorMatcher, AttributeSelectorMatcherValue,
    AttributeSelectorValue, Combinator, CombinatorValue, ComplexSelector, ComplexSelectorChildren,
    ComponentValue, CompoundSelector, Declaration, DeclarationName, Ident, NestingSelector,
    PseudoClassSelector, PseudoClassSelectorChildren, QualifiedRule, QualifiedRulePrelude,
    SelectorList, SimpleBlock, SubclassSelector, Token, TokenAndSpan, TypeSelector,
    UniversalSelector, WqName,
};
use swc_css_visit::{VisitMut, VisitMutWith};

pub struct CSSRTLCompilerVisitor {}

impl CSSRTLCompilerVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

lazy_static! {
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
                                            // [dir=ltr]
                                            ComplexSelector {
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
                                                                        value: Some(AttributeSelectorValue::Ident(Ident {
                                                                            span: DUMMY_SP,
                                                                            value: Atom::new("ltr"),
                                                                            raw: Some(Atom::new(
                                                                                "ltr"
                                                                            ))
                                                                        })),
                                                                        modifier: None
                                                                    }
                                                                ))
                                                            ]
                                                        }
                                                    )
                                                ]
                                            },
                                            // [dir=ltr] *
                                            ComplexSelector {
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
                                                                        value: Some(AttributeSelectorValue::Ident(Ident {
                                                                            span: DUMMY_SP,
                                                                            value: Atom::new("ltr"),
                                                                            raw: Some(Atom::new(
                                                                                "ltr"
                                                                            ))
                                                                        })),
                                                                        modifier: None
                                                                    }
                                                                ))
                                                            ]
                                                        }
                                                    ),
                                                    ComplexSelectorChildren::Combinator(Combinator {
                                                        span: DUMMY_SP,
                                                        value: CombinatorValue::Descendant
                                                    }),
                                                    ComplexSelectorChildren::CompoundSelector(CompoundSelector{
                                                        span: DUMMY_SP,
                                                        nesting_selector: None,
                                                        type_selector: Some(Box::new(TypeSelector::Universal(UniversalSelector {span:DUMMY_SP, prefix: None }))),
                                                        subclass_selectors:vec![]
                                                    })
                                                ]
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
                                raw: Some(Atom::new("where"))
                            },
                            children: Some(vec![PseudoClassSelectorChildren::SelectorList(
                                SelectorList {
                                    span: DUMMY_SP,
                                    children: vec![
                                        // [dir=rtl]
                                        ComplexSelector {
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
                                                                value: Some(AttributeSelectorValue::Ident(Ident {
                                                                    span: DUMMY_SP,
                                                                    value: Atom::new("rtl"),
                                                                    raw: Some(Atom::new(
                                                                        "rtl"
                                                                    ))
                                                                })),
                                                                modifier: None
                                                            }
                                                        ))
                                                    ]
                                                }
                                            )
                                        ]
                                    },
                                        // [dir=rtl] *
                                    ComplexSelector {
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
                                                                value: Some(AttributeSelectorValue::Ident(Ident {
                                                                    span: DUMMY_SP,
                                                                    value: Atom::new("rtl"),
                                                                    raw: Some(Atom::new(
                                                                        "rtl"
                                                                    ))
                                                                })),
                                                                modifier: None
                                                            }
                                                        ))
                                                    ]
                                                }
                                            ),
                                            ComplexSelectorChildren::Combinator(Combinator {
                                                span: DUMMY_SP,
                                                value: CombinatorValue::Descendant
                                            }),
                                            ComplexSelectorChildren::CompoundSelector(CompoundSelector{
                                                span: DUMMY_SP,
                                                nesting_selector: None,
                                                type_selector: Some(Box::new(TypeSelector::Universal(UniversalSelector {span:DUMMY_SP, prefix: None }))),
                                                subclass_selectors:vec![]
                                            })
                                        ]
                                    }]
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
            "right" | "left" | "margin-right" | "margin-left" | "padding-right"
            | "padding-left" => {
                let rtl_name = match ident.value.to_ascii_lowercase().as_ref() {
                    "right" => "left",
                    "left" => "right",
                    "margin-right" => "margin-left",
                    "margin-left" => "margin-right",
                    "padding-right" => "padding-left",
                    "padding-left" => "padding-right",
                    _ => "",
                };
                rtl_vec.push(ComponentValue::Declaration(Box::new(Declaration {
                    name: DeclarationName::Ident(Ident {
                        span: decl.span.clone(),
                        value: Atom::new(rtl_name),
                        raw: Some(Atom::new(rtl_name)),
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
            "direction" => {
                if decl.value.len() == 1 {
                    match decl.value.get(0).unwrap() {
                        ComponentValue::Ident(ident) if ident.value == "ltr" => {
                            rtl_vec.push(ComponentValue::Declaration(Box::new(Declaration {
                                name: decl.name.clone(),
                                value: vec![ComponentValue::Ident(Box::new(Ident {
                                    span: decl.span.clone(),
                                    value: "rtl".into(),
                                    raw: Some("rtl".into()),
                                }))],
                                span: decl.span.clone(),
                                important: decl.important.clone(),
                            })));
                            ltr_vec.push(ComponentValue::Declaration(Box::new(decl)));
                            None
                        }
                        ComponentValue::Ident(ident) if ident.value == "rtl" => {
                            rtl_vec.push(ComponentValue::Declaration(Box::new(Declaration {
                                name: decl.name.clone(),
                                value: vec![ComponentValue::Ident(Box::new(Ident {
                                    span: decl.span.clone(),
                                    value: "ltr".into(),
                                    raw: Some("ltr".into()),
                                }))],
                                span: decl.span.clone(),
                                important: decl.important.clone(),
                            })));
                            ltr_vec.push(ComponentValue::Declaration(Box::new(decl)));
                            None
                        }
                        _ => Some(decl),
                    }
                } else {
                    Some(decl)
                }
            }
            _ => Some(decl),
        },
        _ => Some(decl),
    }
}

fn convert_block(block: &mut SimpleBlock) {
    let old_value = std::mem::replace(&mut block.value, Vec::<ComponentValue>::new());
    let new_value = &mut block.value;

    let mut ltr_vec: Vec<ComponentValue> = vec![];
    let mut rtl_vec: Vec<ComponentValue> = vec![];

    for val in old_value {
        match val {
            ComponentValue::Declaration(decl) => {
                if let Some(decl) = convert_declaration(*decl, &mut ltr_vec, &mut rtl_vec) {
                    new_value.push(ComponentValue::Declaration(Box::new(decl)));
                }
            }
            _ => new_value.push(val),
        }
    }

    if ltr_vec.len() > 0 {
        new_value.push(ComponentValue::QualifiedRule(Box::new(QualifiedRule {
            prelude: LTR_PRELUDE.clone(),
            block: SimpleBlock {
                span: DUMMY_SP,
                name: TokenAndSpan {
                    span: DUMMY_SP,
                    token: Token::LBrace,
                },
                value: ltr_vec,
            },
            span: DUMMY_SP,
        })));
    }
    if rtl_vec.len() > 0 {
        new_value.push(ComponentValue::QualifiedRule(Box::new(QualifiedRule {
            prelude: RTL_PRELUDE.clone(),
            block: SimpleBlock {
                span: DUMMY_SP,
                name: TokenAndSpan {
                    span: DUMMY_SP,
                    token: Token::LBrace,
                },
                value: rtl_vec,
            },
            span: DUMMY_SP,
        })));
    }
}

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
