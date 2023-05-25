extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Rust Calculator");
    window.set_default_size(300, 400);
    window.set_position(gtk::WindowPosition::Center);
    window.set_border_width(10);

    // Create a button grid
    let grid = gtk::Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);

    // Create the calculator display
    let display = gtk::Entry::new();
    display.set_editable(false);
    display.set_alignment(1.0);

    // Add the display to the grid
    grid.attach(&display, 0, 0, 4, 1);

    // Create number buttons
    let numbers = ["7", "8", "9", "4", "5", "6", "1", "2", "3", "0"];
    let mut button_id = 0;
    for i in 1..=4 {
        for j in 0..=2 {
            let button = Button::new_with_label(numbers[button_id]);
            button_id += 1;
            button.set_hexpand(true);
            button.set_vexpand(true);
            grid.attach(&button, j, i, 1, 1);

            // Connect button press event
            button.connect_clicked(move |_| {
                let text = display.get_text().unwrap().to_string();
                display.set_text(&(text + button.get_label().unwrap()));
            });
        }
    }

    // Create the equals button
    let equals_button = Button::new_with_label("=");
    equals_button.set_hexpand(true);
    equals_button.set_vexpand(true);
    grid.attach(&equals_button, 3, 4, 1, 1);

    // Connect equals button press event
    equals_button.connect_clicked(move |_| {
        let text = display.get_text().unwrap().to_string();
        let result = match eval::eval(&text) {
            Ok(value) => value.to_string(),
            Err(err) => err.to_string(),
        };
        display.set_text(&result);
    });

    // Create the clear button
    let clear_button = Button::new_with_label("C");
    clear_button.set_hexpand(true);
    clear_button.set_vexpand(true);
    grid.attach(&clear_button, 2, 4, 1, 1);

    // Connect clear button press event
    clear_button.connect_clicked(move |_| {
        display.set_text("");
    });

    // Create the main vertical box layout
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.set_homogeneous(false);
    vbox.add(&grid);

    // Add the main layout to the window
    window.add(&vbox);

    // Handle window close event
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all the widgets
    window.show_all();

    // Start the GTK main event loop
    gtk::main();
}
