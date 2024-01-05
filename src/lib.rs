use aho_corasick::AhoCorasick;
use std::{fmt::Display, io::Write};
extern crate self as hyped;

fn escape(html: impl Display) -> String {
    let patterns = &["&", "<", ">", "\"", "'"];
    let replace_with = &["&amp;", "&lt;", "&gt;", "&quot;", "&#39;"];

    AhoCorasick::new(patterns)
        .unwrap()
        .replace_all(&html.to_string(), replace_with)
}

pub struct Element {
    name: &'static str,
    attrs: String,
    children: Option<Box<dyn Render>>,
}

macro_rules! impl_attr {
    ($ident:ident) => {
        pub fn $ident(self, value: impl Display) -> Self {
            self.attr(stringify!($ident), escape(value))
        }
    };

    ($ident:ident, $name:expr) => {
        pub fn $ident(self, value: impl Display) -> Self {
            self.attr($name, escape(value))
        }
    };
}

macro_rules! impl_bool_attr {
    ($ident:ident) => {
        pub fn $ident(self) -> Self {
            self.bool_attr(stringify!($ident))
        }
    };
}

impl Element {
    fn new(name: &'static str, children: Option<Box<dyn Render>>) -> Element {
        Element {
            name,
            attrs: String::default(),
            children,
        }
    }

    pub fn attr(mut self, name: &'static str, value: impl Display) -> Self {
        if !self.attrs.is_empty() {
            self.attrs.push_str(" ");
        }
        self.attrs.push_str(name);
        self.attrs.push_str("=\"");
        self.attrs.push_str(&escape(value));
        self.attrs.push_str("\"");

        self
    }

    pub fn bool_attr(mut self, name: &'static str) -> Self {
        if !self.attrs.is_empty() {
            self.attrs.push_str(" ");
        }
        self.attrs.push_str(name);

        self
    }

    impl_attr!(class);
    impl_attr!(id);
    impl_attr!(charset);
    impl_attr!(content);
    impl_attr!(name);
    impl_attr!(href);
    impl_attr!(rel);
    impl_attr!(src);
    impl_attr!(integrity);
    impl_attr!(crossorigin);
    impl_attr!(role);
    impl_attr!(method);
    impl_attr!(action);
    impl_attr!(placeholder);
    impl_attr!(value);
    impl_attr!(rows);
    impl_attr!(alt);
    impl_attr!(style);
    impl_attr!(onclick);
    impl_attr!(placement);
    impl_attr!(toggle);
    impl_attr!(scope);
    impl_attr!(title);
    impl_attr!(r#type, "type");
    impl_attr!(r#for, "for");
    impl_attr!(aria_controls, "aria-controls");
    impl_attr!(aria_expanded, "aria-expanded");
    impl_attr!(aria_label, "aria-label");
    impl_attr!(aria_haspopup, "aria-haspopup");
    impl_attr!(aria_labelledby, "aria-labelledby");
    impl_attr!(aria_current, "aria-current");
    impl_bool_attr!(defer);
    impl_bool_attr!(checked);
    impl_bool_attr!(enabled);
    impl_bool_attr!(disabled);
}

pub trait Render {
    fn render(&self, buffer: &mut Vec<u8>) -> std::io::Result<()>;
}

impl Render for Element {
    fn render(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        buffer.write(b"<")?;
        buffer.write(self.name.as_bytes())?;
        if !self.attrs.is_empty() {
            buffer.write(b" ")?;
            buffer.write(self.attrs.as_bytes())?;
        }
        buffer.write(b">")?;
        match &self.children {
            Some(children) => {
                children.render(buffer)?;
                buffer.write(b"</")?;
                buffer.write(self.name.as_bytes())?;
                buffer.write(b">")?;
            }
            None => {}
        };

        Ok(())
    }
}

impl Render for String {
    fn render(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        buffer.write(escape(self).as_bytes())?;

        Ok(())
    }
}

impl<'a> Render for &'a str {
    fn render(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
        buffer.write(escape(self).as_bytes())?;

        Ok(())
    }
}

impl Render for () {
    fn render(&self, _buffer: &mut Vec<u8>) -> std::io::Result<()> {
        Ok(())
    }
}

macro_rules! impl_render_tuple {
    ($max:expr) => {
        seq_macro::seq!(N in 0..=$max {
            impl<#(T~N,)*> Render for (#(T~N,)*)
            where
                #(T~N: Render,)*
            {
                fn render(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
                    #(self.N.render(buffer)?;)*

                    Ok(())
                }
            }
        });
    };
}

seq_macro::seq!(N in 0..=31 {
    impl_render_tuple!(N);
});

pub fn doctype() -> Element {
    Element::new("!DOCTYPE html", None)
}

pub fn render(renderable: impl Render + 'static) -> String {
    let mut v: Vec<u8> = vec![];
    renderable.render(&mut v).expect("Failed to render html");
    String::from_utf8_lossy(&v).into()
}

macro_rules! impl_render_num {
    ($t:ty) => {
        impl Render for $t {
            fn render(&self, buffer: &mut Vec<u8>) -> std::io::Result<()> {
                buffer.write(self.to_string().as_bytes())?;
                Ok(())
            }
        }
    };
}

impl_render_num!(u16);
impl_render_num!(f64);
impl_render_num!(f32);
impl_render_num!(i64);
impl_render_num!(u64);
impl_render_num!(i32);
impl_render_num!(u32);
impl_render_num!(usize);
impl_render_num!(isize);

pub fn element(name: &'static str, children: impl Render + 'static) -> Element {
    Element::new(name, Some(Box::new(children)))
}

pub fn self_closing_element(name: &'static str) -> Element {
    Element::new(name, None)
}

macro_rules! impl_element {
    ($ident:ident) => {
        pub fn $ident(child: impl Render + 'static) -> Element {
            Element::new(stringify!($ident), Some(Box::new(child)))
        }
    };
}

macro_rules! impl_self_closing_element {
    ($ident:ident) => {
        pub fn $ident() -> Element {
            Element::new(stringify!($ident), None)
        }
    };
}

impl_element!(html);
impl_element!(head);
impl_element!(title);
impl_element!(body);
impl_element!(div);
impl_element!(section);
impl_element!(h1);
impl_element!(h2);
impl_element!(h3);
impl_element!(h4);
impl_element!(h5);
impl_element!(li);
impl_element!(ul);
impl_element!(ol);
impl_element!(p);
impl_element!(span);
impl_element!(b);
impl_element!(i);
impl_element!(u);
impl_element!(tt);
impl_element!(string);
impl_element!(pre);
impl_element!(script);
impl_element!(main);
impl_element!(nav);
impl_element!(a);
impl_element!(form);
impl_element!(button);
impl_element!(blockquote);
impl_element!(footer);
impl_element!(wrapper);
impl_element!(label);
impl_element!(table);
impl_element!(thead);
impl_element!(th);
impl_element!(tr);
impl_element!(td);
impl_element!(tbody);
impl_element!(textarea);
impl_element!(datalist);
impl_element!(option);
impl_element!(link);

impl_self_closing_element!(input);
impl_self_closing_element!(meta);
impl_self_closing_element!(img);
impl_self_closing_element!(br);

#[cfg(test)]
mod tests {
    use hyped::*;

    #[test]
    fn it_works() {
        let html = render((doctype(), html((head(()), body(())))));
        assert_eq!(
            "<!DOCTYPE html><html><head></head><body></body></html>",
            html
        );
    }

    #[test]
    fn it_works_with_numbers() {
        let html = render((doctype(), html((head(()), body(0)))));
        assert_eq!(
            "<!DOCTYPE html><html><head></head><body>0</body></html>",
            html
        );
    }

    #[test]
    fn it_escapes_correctly() {
        let html = render((doctype(), html((head(()), body("<div />")))));
        assert_eq!(
            html,
            "<!DOCTYPE html><html><head></head><body>&lt;div /&gt;</body></html>",
        );
    }

    #[test]
    fn it_escapes_more() {
        let html = render((
            doctype(),
            html((head(()), body("<script>alert('hello')</script>"))),
        ));
        assert_eq!(
            html,
            "<!DOCTYPE html><html><head></head><body>&lt;script&gt;alert(&#39;hello&#39;)&lt;/script&gt;</body></html>",
        );
    }

    #[test]
    fn it_renders_attributes() {
        let html = render((doctype(), html((head(()), body(div("hello").id("hello"))))));
        assert_eq!(
            "<!DOCTYPE html><html><head></head><body><div id=\"hello\">hello</div></body></html>",
            html
        );
    }

    #[test]
    fn it_renders_custom_elements() {
        fn turbo_frame(children: impl Render + 'static) -> Element {
            element("turbo-frame", children)
        }
        let html = render(turbo_frame(div("inside turbo frame")).id("id"));
        assert_eq!(
            "<turbo-frame id=\"id\"><div>inside turbo frame</div></turbo-frame>",
            html
        );
    }

    #[test]
    fn it_renders_custom_self_closing_elements() {
        fn hx_close() -> Element {
            self_closing_element("hx-close")
        }
        let html = render(hx_close().id("id"));
        assert_eq!("<hx-close id=\"id\">", html);
    }

    #[test]
    fn readme_works() {
        use hyped::*;

        fn render_to_string(element: Element) -> String {
            render((
                doctype(),
                html((
                    head((title("title"), meta().charset("utf-8"))),
                    body(element),
                )),
            ))
        }

        assert_eq!(
        render_to_string(div("hyped")),
        "<!DOCTYPE html><html><head><title>title</title><meta charset=\"utf-8\"></head><body><div>hyped</div></body></html>"
      )
    }

    #[test]
    fn max_tuples_works() {
        let elements = seq_macro::seq!(N in 0..=31 {
            (#(br().id(N),)*)
        });

        assert_eq!(render(elements),
            "<br id=\"0\"><br id=\"1\"><br id=\"2\"><br id=\"3\"><br id=\"4\"><br id=\"5\"><br id=\"6\"><br id=\"7\"><br id=\"8\"><br id=\"9\"><br id=\"10\"><br id=\"11\"><br id=\"12\"><br id=\"13\"><br id=\"14\"><br id=\"15\"><br id=\"16\"><br id=\"17\"><br id=\"18\"><br id=\"19\"><br id=\"20\"><br id=\"21\"><br id=\"22\"><br id=\"23\"><br id=\"24\"><br id=\"25\"><br id=\"26\"><br id=\"27\"><br id=\"28\"><br id=\"29\"><br id=\"30\"><br id=\"31\">"
        )
    }

    #[test]
    fn bool_attr_works() {
        let html = render(input().r#type("checkbox").checked());

        assert_eq!(html, r#"<input type="checkbox" checked>"#)
    }

    #[test]
    fn multiple_attrs_spaced_correctly() {
        let html = render(input().r#type("checkbox").checked().aria_label("label"));

        assert_eq!(
            html,
            r#"<input type="checkbox" checked aria-label="label">"#
        )
    }
}
