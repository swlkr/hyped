# hyped

hyped offers an ergonomic way to render html from plain rust functions

```sh
cargo add hyped
```

# Write some html

```rust
use hyped::*;

fn render_to_string(element: Element) -> String {
  render((
    doctype(),
    html((
      head((title("title"), meta().charset("utf-8"))),
      body(element)
    ))
  ))
}

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() {
      assert_eq!(
        render_to_string(div("hyped")),
        "<!DOCTYPE html><html><head><title>title</title></head><body><div>hyped</div></body></html>"
      )
  }
}
```
