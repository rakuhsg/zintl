# 🚧 WIP: Zintl - Building GUI with Rust

| [Source code](https://github.com/rakuhsg/zintl) | [Crates.io](https://crates.io/crates/zintl) | [Docs](https://docs.rs/zintl/latest/zintl) |

### Stateful counter app example

```rs
use zintl::*;

#[derive(Default)]
struct HelloWorld {
    count: usize,
    context: Context,
}

impl HelloWorld {
    pub fn new(count: usize) -> Self {
        HelloWorld {
            count,
            ...Default::default(),
        }
    }
}

impl Composable for HelloWorld {
    fn compose(&self) -> impl View {
        VStack::new().children(v![
            Label::new("Hello, world!"),
            Label::new(format!("Count: {}", self.count),
            Button::new("Increment").on_click(||
                self.count += 1;
            })
        ])
    }
}

fn main() {
    let app = App::new(
        StatefulView::new::<usize>(marker!(), 0, |value| {
            HelloWorld::new(value)
        }),
    );
    run_app(app);
}
```

## Stay simple, go fast
