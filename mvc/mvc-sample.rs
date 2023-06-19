pub struct Model {
    data: i32,
}

impl Model {
    fn new() -> Model {
        Model { data: 0 }
    }

    fn increment(&mut self) {
        self.data += 1;
    }

    fn value(&self) -> i32 {
        self.data
    }
}

pub struct View {
    display: String,
}

impl View {
    fn new() -> View {
        View { display: String::new() }
    }

    fn update(&mut self, data: i32) {
        self.display = format!("Current value: {}", data);
    }

    fn display(&self) {
        println!("{}", self.display);
    }
}

pub struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    fn new() -> Controller {
        Controller {
            model: Model::new(),
            view: View::new(),
        }
    }

    fn increment(&mut self) {
        self.model.increment();
        self.view.update(self.model.value());
    }

    fn display(&self) {
        self.view.display();
    }
}

fn main() {
    let mut controller = Controller::new();
    controller.display();
    controller.increment();
    controller.display();
}
