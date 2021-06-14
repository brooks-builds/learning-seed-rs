// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        tasks: vec![
            Task {
                name: "I am a task".into(),
            },
            Task {
                name: "Pet the cat".into(),
            },
        ],
        input: "".into(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    tasks: Vec<Task>,
    input: String,
}

struct Task {
    name: String,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    AddTask,
    UpdateTaskInput(String),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UpdateTaskInput(task) => model.input = task,
        Msg::AddTask => {
            model.tasks.push(Task {
                name: model.input.clone(),
            });
            model.input.clear();
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    main![
        h1!["Todo"],
        h2!["Create a task"],
        input![
            attrs![
                At::Value => model.input,
            ],
            input_ev(Ev::Input, |new_task| { Msg::UpdateTaskInput(new_task) }),
            input_ev(Ev::KeyDown, |event| Msg::AddTask),
        ],
        button!["add task", ev(Ev::Click, |_| Msg::AddTask)],
        h2!["Tasks"],
        ul![model.tasks.iter().map(|task| li!(&task.name))]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
