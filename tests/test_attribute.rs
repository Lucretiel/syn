#[macro_use]
mod macros;

use syn::parse::Parser;
use syn::{Attribute, Meta};

#[test]
fn test_meta_item_word() {
    let meta = test("#[foo]");

    snapshot!(meta, @r###"
    Path(Path {
        segments: [
            PathSegment {
                ident: "foo",
                arguments: None,
            },
        ],
    })
    "###);
}

#[test]
fn test_meta_item_name_value() {
    let meta = test("#[foo = 5]");

    snapshot!(meta, @r###"
    Meta::NameValue {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        value: Expr::Lit {
            lit: 5,
        },
    }
    "###);
}

#[test]
fn test_meta_item_bool_value() {
    let meta = test("#[foo = true]");

    snapshot!(meta, @r###"
    Meta::NameValue {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        value: Expr::Lit {
            lit: Lit::Bool {
                value: true,
            },
        },
    }
    "###);

    let meta = test("#[foo = false]");

    snapshot!(meta, @r###"
    Meta::NameValue {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        value: Expr::Lit {
            lit: Lit::Bool {
                value: false,
            },
        },
    }
    "###);
}

#[test]
fn test_meta_item_list_lit() {
    let meta = test("#[foo(5)]");

    snapshot!(meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`5`),
    }
    "###);
}

#[test]
fn test_meta_item_list_word() {
    let meta = test("#[foo(bar)]");

    snapshot!(meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`bar`),
    }
    "###);
}

#[test]
fn test_meta_item_list_name_value() {
    let meta = test("#[foo(bar = 5)]");

    snapshot!(meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`bar = 5`),
    }
    "###);
}

#[test]
fn test_meta_item_list_bool_value() {
    let meta = test("#[foo(bar = true)]");

    snapshot!(meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`bar = true`),
    }
    "###);
}

#[test]
fn test_meta_item_multiple() {
    let meta = test("#[foo(word, name = 5, list(name2 = 6), word2)]");

    snapshot!(meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`word , name = 5 , list (name2 = 6) , word2`),
    }
    "###);
}

#[test]
fn test_bool_lit() {
    let meta = test("#[foo(true)]");

    snapshot!(meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "foo",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`true`),
    }
    "###);
}

#[test]
fn test_negative_lit() {
    let meta = test("#[form(min = -1, max = 200)]");

    snapshot!(meta, @r###"
    Meta::List {
        path: Path {
            segments: [
                PathSegment {
                    ident: "form",
                    arguments: None,
                },
            ],
        },
        delimiter: Paren,
        tokens: TokenStream(`min = - 1 , max = 200`),
    }
    "###);
}

fn test(input: &str) -> Meta {
    let attrs = Attribute::parse_outer.parse_str(input).unwrap();

    assert_eq!(attrs.len(), 1);
    let attr = attrs.into_iter().next().unwrap();

    attr.meta
}
