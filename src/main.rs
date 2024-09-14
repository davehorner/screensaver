
use nannou::prelude::*;
use std::process::exit;

struct Model {
    window_ids: Vec<WindowId>,  // Store multiple window IDs
    positions: Vec<Option<Point2>>,  // Store positions for each window's mouse events
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let mut window_ids = Vec::new();
    let mut positions = Vec::new();
    let monitors = app.available_monitors();

    // Convert monitors to an iterator and enumerate
    for (i, monitor) in monitors.into_iter().enumerate() {
        let position = monitor.position(); // Get the position of the monitor

        let window_id = app
            .new_window()
            .key_pressed(key_pressed)  // Use a function pointer directly
            .mouse_moved(mouse_moved)  // Use a function pointer directly
            .fullscreen()
            .view(view_circles)  // Associate view_circles with each window
            .build()
            .unwrap();

        // Set window position using the underlying winit window.
        if let Some(window) = app.window(window_id) {
            let winit_window = window.winit_window();
            winit_window.set_outer_position(nannou::winit::dpi::PhysicalPosition {
                x: position.x,
                y: position.y,
            });
        }

        window_ids.push(window_id);
        positions.push(None);  // Initialize the mouse position for each window
    }

    Model {
        window_ids,
        positions,
    }
}

// This is called periodically to handle updates, such as user inputs or animations
fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    let window_index = get_window_index(app, model);  // Get the window index

   match key {
        Key::RAlt => println!("RAlt key pressed on window {}", window_index),
        _ => exit(0),
    } 
    println!("Key pressed on window {} {:?}", window_index, key);
}

fn mouse_moved(app: &App, model: &mut Model, pos: Point2) {
    let window_index = get_window_index(app, model);  // Get the window index

    // Handle mouse moved events for each window
    if model.positions[window_index].is_none() {
        model.positions[window_index] = Some(pos)
    } else if (model.positions[window_index].unwrap() - pos).length() > 10.0 {
        println!("Mouse moved significantly on window {}", window_index);
        exit(0);
    }
}

fn get_window_index(app: &App, model: &Model) -> usize {
    let current_window_id = app.main_window().id();
    model
        .window_ids
        .iter()
        .position(|&id| id == current_window_id)
        .unwrap_or(0) // Default to 0 if not found
}

fn view_circles(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    // Clear the current frame for the specific window being drawn
    frame.clear(BLACK);

    // Loop through each window to draw
    for (index, window_id) in model.window_ids.iter().enumerate() {
        if let Some(window) = app.window(*window_id) {
            // Get the window's frame
            let draw = app.draw();
            let win_rect = window.rect(); // Get the window's dimensions
            let width = win_rect.w();
            let height = win_rect.h();
            let in_a_row = 15;
            let in_a_column = 10;
            let time = app.time / 1.5;

            // Draw a semi-transparent background rectangle to simulate animation trails
            draw.rect().w_h(width, height).color(rgba(0.0, 0.0, 0.0, 0.2));

            // Draw circles in each window
            for i in 0..in_a_row {
                for j in 0..in_a_column {
                    let i = i as f32;
                    let j = j as f32;
                    let x = width / (in_a_row - 1) as f32 * i - width / 2.0;
                    let y = height / (in_a_column - 1) as f32 * j - height / 2.0;
                    let r = width * 0.01 * ((time + i * 0.5 + j * 0.25).sin() + 1.1) * (i + j) / 2.0;
                    let red = (time * 1.2).sin();
                    let green = (time * 1.5 + i * 0.5 + j * 0.5).sin();
                    let blue = (time - i * 0.5 - j * 0.5).sin();

                    draw.ellipse()
                        .x_y(x, y)
                        .radius(r)
                        .color(rgb((blue + red + green) * 0.2, green * 0.2, blue * 0.9));
                }
            }

            // Finish drawing and render everything to the frame
            draw.to_frame(app, &frame).unwrap();
        }
    }
}
