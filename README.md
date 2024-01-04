# hypertext

hypertext offers an ergonomic way to render html from plain rust functions

```sh
cargo add hypertext # not on crates.io yet, add { git = "https://github.com/swlkr/hypertext" } to your Cargo.toml to use
```

# Write some html

```rust
use hypertext::{*, self};

fn head() -> Element {
  hypertext::head((
    title("title")
  ))
}

fn body(element: Element) -> Element {
  hypertext::body(element)
}

fn html(element: Element) -> String {
    render((doctype(), hypertext::html((head(), body(element)))))
}

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() {
      assert_eq!(html(div("hypertext")), "<!DOCTYPE html><html><head><title>title</title></head><body><div>hypertext</div></body></html>")
  }
}
```
