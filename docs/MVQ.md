# Model-View-Query Architecture

## Example Code

```rust
struct Task {
	id: usize,
	name: String,
}

impl Task {
	fn new(id: usize, name: String) -> Self {
		Task { id, name }
	}
}

impl View<Event> for Task {
	fn render(&self, cx: &ViewContext) -> impl View<Event> {
		HStack::new(v![
			InputBinder(Input::new(), |i| {
				Button::new("Add").on_click(|| {
					cx.query(TodoAppQuery::Done(self.id))
				})
			})
		])
	}
}

enum TodoAppQuery {
	Add(String),
	Done(usize),
}

struct TodoApp {
	tasks: Vec<Task>,
	new_task_id: usize,
}

impl View<Event> for TodoApp {
	fn render(&self, cx: &ViewContext) -> impl View<Event> {
		VStack::from_vec(tasks.clone())
	}
}

impl Model<TodoAppQuery> for TodoApp {
	fn on(&mut self, q: TodoAppQuery, cx: &ModelContext) {
		match q {
			Add(name) => {
				tasks.push(task.clone());
				cx.update();
			}
			Done(id) => {
				let idx = tasks.iter().position(|t| t.id == id).unwrap();
				tasks.remove(idx);
				cx.update();
			}
		}
	}

	fn children(&self) -> impl IntoChildren {
	}
}
```

## Model and Query

### Model

Model retains states and children.

## View

## Global Model
