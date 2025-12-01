/*
Made by: Mathew Dusome
May 11, 2025
Program Details: Scale module for handling screen scaling and virtual resolution

To import you need:

In your mod.rs file located in the modules folder add the following to the end of the file:
    pub mod scale;
In the Cargo.toml file add the following:
    [features]
    scale = []
    default = ["scale"]

Then in your main.rs file add the following to the top of the file:
    
Then add the following with the use commands:
use crate::modules::scale::use_virtual_resolution;

Usage examples:
1. Setting up virtual resolution in your game loop:
    loop {
        // Set the virtual resolution to 1024x768
        use_virtual_resolution(1024.0, 768.0);
        // Rest of your game code...
        clear_background(DARKGREEN);
        // Draw your game objects...
    }

Benefits:
- Your game will maintain the correct aspect ratio on any screen size
- All game coordinates stay consistent regardless of the physical screen resolution
- UI elements and interaction work correctly on different devices
- Content is automatically centered with letterboxing when needed
*/

use macroquad::prelude::*;
use std::cell::RefCell;

// Static variable to store the camera using RefCell for interior mutability
thread_local! {
    static CAMERA: RefCell<Camera2D> = RefCell::new(Camera2D {
        zoom: vec2(1.0, 1.0),
        target: vec2(0.0, 0.0),
        ..Default::default()
    });
    
    // We'll store the current virtual resolution here - made pub so other modules can access it
    pub static VIRTUAL_RESOLUTION: RefCell<(f32, f32)> = RefCell::new((1024.0, 768.0));
}

/// Sets the camera to the virtual resolution and adjusts the scale
pub fn use_virtual_resolution(virtual_width: f32, virtual_height: f32) {
    // Store the virtual resolution for other functions to use
    VIRTUAL_RESOLUTION.with(|res| {
        *res.borrow_mut() = (virtual_width, virtual_height);
    });
    
    let screen_aspect = screen_width() / screen_height();
    let virtual_aspect = virtual_width / virtual_height;

    let (cam_width, cam_height) = if screen_aspect > virtual_aspect {
        // Screen is wider — match height
        let height = virtual_height;
        let width = height * screen_aspect;
        (width, height)
    } else {
        // Screen is taller — match width
        let width = virtual_width;
        let height = width / screen_aspect;
        (width, height)
    };

    CAMERA.with(|camera| {
        let mut camera = camera.borrow_mut();

        *camera = Camera2D {
            zoom: vec2(2.0 / cam_width, 2.0 / cam_height),
            target: vec2(virtual_width / 2.0, virtual_height / 2.0),
            ..Default::default()
        };

        // Apply the camera settings to the game screen
        set_camera(&*camera);
    });
}

/// Function to get the mouse position in world coordinates based on the current camera state
pub fn mouse_position_world() -> (f32, f32) {
    let (mouse_x, mouse_y) = ::macroquad::input::mouse_position();  // Get the raw mouse position

    VIRTUAL_RESOLUTION.with(|res| {
        let (virtual_width, virtual_height) = *res.borrow();
        
        // Get screen dimensions
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Calculate the scale factor between screen and virtual resolution
        let screen_aspect = screen_width / screen_height;
        let virtual_aspect = virtual_width / virtual_height;
        
        let scale_factor = if screen_aspect > virtual_aspect {
            // Screen is wider than virtual - height is matched
            screen_height / virtual_height
        } else {
            // Screen is taller than virtual - width is matched
            screen_width / virtual_width
        };

        // Calculate the offset (to center content)
        let offset_x = (screen_width - virtual_width * scale_factor) / 2.0;
        let offset_y = (screen_height - virtual_height * scale_factor) / 2.0;

        // Convert screen coordinates to virtual coordinates
        let virtual_x = (mouse_x - offset_x) / scale_factor;
        let virtual_y = (mouse_y - offset_y) / scale_factor;

        // Clamp coordinates to the virtual resolution
        let virtual_x = virtual_x.clamp(0.0, virtual_width);
        let virtual_y = virtual_y.clamp(0.0, virtual_height);

        (virtual_x, virtual_y)
    })
}
