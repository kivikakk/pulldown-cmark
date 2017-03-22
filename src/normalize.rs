extern crate html5ever;

use std::default::Default;

use self::html5ever::tendril::TendrilSink;
use self::html5ever::driver::ParseOpts;
use self::html5ever::tree_builder::TreeBuilderOpts;
use self::html5ever::parse_document;
use self::html5ever::serialize::{SerializeOpts, serialize};
use self::html5ever::rcdom::{RcDom, Node, NodeEnum};

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
    match n.node {
        NodeEnum::Document => { },
        NodeEnum::Doctype(..) => { },
        NodeEnum::Text(_) => { },
        NodeEnum::Comment(_) => { },
        NodeEnum::Element(_, _, ref mut attrs) => {
            attrs.sort();
        },
    }

    for h in &n.children {
        process(&mut *h.borrow_mut());
    }
}
