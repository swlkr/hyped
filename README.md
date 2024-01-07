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

# Custom attributes

```rust
use hyped::*;

fn main() {
  input()
    .attr("hx-post", "/")
    .attr("hx-target", ".target")
    .attr("hx-swap", "outerHTML")
    .attr("hx-push-url", "false")
  // renders
  // <input hx-post="/" hx-target=".target" hx-swap="outerHTML" hx-push-url="false">
}
```

# Custom elements

```rust
use hyped::*;

fn turbo_frame(children: Element) -> Element {
    element("turbo-frame", children)
}

fn main() {
  turbo_frame(div("inside turbo frame")).id("id")
  // renders
  // <turbo-frame id="id">
  //   <div>inside turbo frame</div>
  // </turbo-frame>
}
```
