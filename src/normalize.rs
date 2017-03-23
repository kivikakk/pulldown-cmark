use std::default::Default;
use std::fmt::Write;
use std::collections::HashSet;

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

    process(&mut *dom.document.borrow_mut(), None, None);

    let mut res = vec![];
    serialize(&mut res, &dom.document, SerializeOpts::default()).unwrap();
    String::from_utf8(res).unwrap()
}

fn process(n: &mut Node, prev_el: Option<String>, next_el: Option<String>) {
    lazy_static! {
        static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
        static ref LEADING_WHITESPACE_RE: Regex = Regex::new(r"\A\s+").unwrap();
        static ref TRAILING_WHITESPACE_RE: Regex = Regex::new(r"\s+\z").unwrap();
        static ref BLOCK_TAGS: HashSet<&'static str> =
            ["article", "header", "aside", "hgroup", "blockquote", "hr", "iframe", "body", "li",
            "map", "button", "object", "canvas", "ol", "caption", "output", "col", "p", "colgroup",
            "pre", "dd", "progress", "div", "section", "dl", "table", "td", "dt", "tbody", "embed",
            "textarea", "fieldset", "tfoot", "figcaption", "th", "figure", "thead", "footer", "tr",
            "form", "ul", "h1", "h2", "h3", "h4", "h5", "h6", "video", "script", "style"]
                .iter()
                .cloned()
                .collect();
    }

    match n.node {
        NodeEnum::Document => { },
        NodeEnum::Doctype(..) => { },
        NodeEnum::Text(ref mut t) => {
            let mut s = WHITESPACE_RE.replace_all(&**t, " ").into_owned().to_string();
            if let Some(prev_el_name) = prev_el {
                if BLOCK_TAGS.contains(&*prev_el_name) {
                    s = LEADING_WHITESPACE_RE.replace_all(&s, "").into_owned();
                }
            }
            if let Some(next_el_name) = next_el {
                if BLOCK_TAGS.contains(&*next_el_name) {
                    s = TRAILING_WHITESPACE_RE.replace_all(&s, "").into_owned();
                }
            }
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

    let len = n.children.len();
    for i in 0..len {
        let prev = if i == 0 { None } else { get_element(&n.children[i-1].borrow().node) };
        let next = if i == len - 1 { None } else { get_element(&n.children[i+1].borrow().node) };
        process(&mut *n.children[i].borrow_mut(), prev, next);
    }
}

fn get_element(node: &NodeEnum) -> Option<String> {
    match node {
        &NodeEnum::Element(ref qn, _, _) => Some((&*qn.local).to_string().to_lowercase()),
        _ => None,
    }
}
