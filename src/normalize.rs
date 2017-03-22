use std::default::Default;
use std::fmt::Write;

use regex::Regex;
use html5ever::tendril::TendrilSink;
use html5ever::driver::ParseOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::parse_document;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::rcdom::{RcDom, Node, NodeEnum};

pub fn normalize(s: &str) -> String {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .one(s.as_bytes());

    process(&mut *dom.document.borrow_mut());

    let mut res = vec![];
    serialize(&mut res, &dom.document, SerializeOpts::default()).unwrap();
    String::from_utf8(res).unwrap()
}

fn process(n: &mut Node) {
    lazy_static! {
        static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    }

    match n.node {
        NodeEnum::Document => { },
        NodeEnum::Doctype(..) => { },
        NodeEnum::Text(ref mut t) => {
            let s = &*WHITESPACE_RE.replace_all(&**t, " ").into_owned();
            t.clear();
            t.write_str(&s).unwrap();
        },
        NodeEnum::Comment(_) => { },
        NodeEnum::Element(_, _, ref mut attrs) => {
            for a in attrs.iter_mut() {
                a.name.local = a.name.local.to_ascii_lowercase();
            }
            attrs.sort();
        },
    }

    for h in &n.children {
        process(&mut *h.borrow_mut());
    }
}
