# Zintl.rs - Building GUI with Rust

## Status:WIP

| [Source code](https://github.com/CanvasPads/Zintl) | [Crates.io](https://crates.io/crates/zintl) | [Docs](https://docs.rs/zintl/latest/zintl) |

## Rapidly build GUI applications in Rust

```rs
use zintl::prelude::*;
use zintl_run::run_app;

#[derive(Default)]
struct HelloWorldApp {
    count: usize,
}

impl ComposableView for HelloWorldApp {
    fn compose(&self) -> impl View {
        VStack::new().children([
            Label::new("Hello, world!"),
            Button::new(format!("Click me! {}", self.count))
              .on_click(move |_| {
                  update(move |ctx| {
                      self.count += 1;
                  });
              }),
        ])
    }
}

#[tokio::main]
async fn main() {
    let app = App::new().root(HelloWorldApp::default());
    await run_app(app);
}
```

## Stay simple, go fast
