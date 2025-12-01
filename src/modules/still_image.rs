/*
Made by: Mathew Dusome
May 3, 2025
Program Details: Image module for displaying and manipulating images

To import you need:

In your mod.rs file located in the modules folder add the following to the end of the file
    pub mod still_image;
    
Then add the following with the use commands:
use crate::modules::still_image::StillImage;

Usage examples:
1. Create a new image object:
    let img = StillImage::new(
        "assets/image_name.png",
        100.0,  // width
        200.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

2. Create an empty image to load later:
    // Pass an empty string "" instead of a file path to create a cleared/empty image
    let img = StillImage::new(
        "",     // Empty string creates a transparent image
        100.0,  // width
        200.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    
    // Then later, you can set a texture if you use the texture manager:
    img.set_preload(texture_manager.get_preload("assets/image1.png").unwrap());

3. With custom stretch and zoom options:
    let img_custom = StillImage::new(
        "assets/image_name.png",
        100.0,
        200.0,
        200.0,
        60.0,
        false,  // Disable stretching
        1.5,    // Set zoom to 150%
    ).await;

4. Using with TextureManager:
    // Since all textures are preloaded, you can directly pass the result of get_preload()
    // to set_preload() without intermediate variables:
    img.set_preload(texture_manager.get_preload("assets/image1.png").unwrap());
    
    // The unwrap() is safe because we know the texture was preloaded

5. Clear an image (set to transparent):
    img.clear();
    
6. Draw the image in your game loop:
    img.draw();

Additional functionality:
- Zoom controls: set_zoom(), zoom_in(), zoom_out(), reset_zoom()
- Stretch controls: enable_stretch(), disable_stretch(), toggle_stretch()
- Position control: set_position()
- Check if empty: is_empty()
*/
use macroquad::prelude::*;
use macroquad::texture::Texture2D;

pub struct StillImage {
    texture: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    transparency_mask: Option<Vec<u8>>, // Changed to Option<Vec<u8>> to make it optional
    stretch_enabled: bool, // Flag to control image stretching
    zoom_level: f32, // Zoom factor to scale the image
    filename: String, // Store the original filename/path
    angle: f32, // Angle of rotation
}

impl StillImage {
    // Constructor for ImageStill with asset path and x, y location
    pub async fn new(
        asset_path: &str, 
        width: f32, 
        height: f32, 
        x: f32, 
        y: f32,
        stretch_enabled: bool,
        zoom_level: f32
    ) -> Self {
        // Check if the asset path is empty
        if asset_path.is_empty() {
            // Create an empty/clear image
            let empty_texture = Texture2D::from_rgba8(1, 1, &[0, 0, 0, 0]);
            let empty_mask = Some(vec![0]); // Single transparent pixel
            
            return Self { 
                x, 
                y, 
                width, 
                height, 
                texture: empty_texture, 
                transparency_mask: empty_mask,
                stretch_enabled,
                zoom_level: zoom_level.max(0.1), // Ensure minimum zoom
                filename: "__empty__".to_string(), // Use a special filename
                angle: 0.0, // Default angle
            };
        }
        
        // Normal path for valid asset paths
        let (texture, transparency_mask) = set_texture_main(asset_path).await;
        Self { 
            x, 
            y, 
            width, 
            height, 
            texture, 
            transparency_mask,
            stretch_enabled,
            zoom_level: zoom_level.max(0.1), // Ensure minimum zoom
            filename: asset_path.to_string(), // Store the original filename
            angle: 0.0, // Default angle
        }
    }

    // Method to draw the image with current settings
    pub fn draw(&self) {
        // Get the size to use for drawing
        let (draw_width, draw_height) = if self.stretch_enabled {
            (self.width, self.height)
        } else {
            // Use original texture size when stretch is disabled
            (self.texture.width(), self.texture.height())
        };
        
        // Apply zoom factor
        let final_width = draw_width * self.zoom_level;
        let final_height = draw_height * self.zoom_level;
        
        draw_texture_ex(
            &self.texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                rotation: self.angle,
                dest_size: Some(vec2(final_width, final_height)),
                ..Default::default()
            },
        );
    }

    // Accessors for image properties
    #[allow(unused)]
    pub fn pos(&self) -> Vec2 {
        vec2(self.x, self.y)
    }
    #[allow(unused)]
    pub fn size(&self) -> Vec2 {
        let (width, height) = if self.stretch_enabled {
            (self.width, self.height)
        } else {
            (self.texture.width(), self.texture.height())
        };
        
        vec2(width * self.zoom_level, height * self.zoom_level)
    }
    #[allow(unused)]
    pub fn texture_size(&self) -> Vec2 {
        vec2(self.texture.width(), self.texture.height())
    }
    #[allow(unused)]
    pub fn set_position(&mut self, pos: Vec2) {
        self.x = pos[0];
        self.y = pos[1];
    }
    #[allow(unused)]
    pub fn set_angle(&mut self, x: f32) {
        self.angle = x;
    }
    #[allow(unused)]
    pub fn get_angle(&self) -> f32 {
        self.angle
    }
    // Get and set x position
    #[allow(unused)]
    pub fn get_x(&self) -> f32 {
        self.x
    }

    #[allow(unused)]
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    // Get and set y position
    #[allow(unused)]
    pub fn get_y(&self) -> f32 {
        self.y
    }

    #[allow(unused)]
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    // Get the original filename/path of the loaded image
    #[allow(unused)]
    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    // Get the transparency mask (bitmask)
    #[allow(unused)]
    pub fn get_mask(&self) -> Option<Vec<u8>> {
        self.transparency_mask.clone()
    }

    #[allow(unused)]
    pub async fn set_texture(&mut self, texture_path: &str) {
        let (texture, transparency_mask) = set_texture_main(texture_path).await;
        self.texture = texture;
        self.transparency_mask = transparency_mask;
        self.filename = texture_path.to_string(); // Update the filename when texture changes
    }
    
    // Methods to toggle stretching
    #[allow(unused)]
    pub fn enable_stretch(&mut self) {
        self.stretch_enabled = true;
    }
    
    #[allow(unused)]
    pub fn disable_stretch(&mut self) {
        self.stretch_enabled = false;
    }
    
    #[allow(unused)]
    pub fn toggle_stretch(&mut self) {
        self.stretch_enabled = !self.stretch_enabled;
    }
    
    #[allow(unused)]
    pub fn is_stretch_enabled(&self) -> bool {
        self.stretch_enabled
    }
    
    #[allow(unused)]
    pub fn set_stretch(&mut self, enabled: bool) {
        self.stretch_enabled = enabled;
    }
    
    // Zoom methods
    #[allow(unused)]
    pub fn set_zoom(&mut self, zoom_level: f32) {
        self.zoom_level = zoom_level.max(0.1); // Prevent zoom from going too small
    }
    
    #[allow(unused)]
    pub fn zoom_in(&mut self, amount: f32) {
        self.zoom_level += amount;
        if self.zoom_level < 0.1 {
            self.zoom_level = 0.1; // Minimum zoom level
        }
    }
    
    #[allow(unused)]
    pub fn zoom_out(&mut self, amount: f32) {
        self.zoom_level -= amount;
        if self.zoom_level < 0.1 {
            self.zoom_level = 0.1; // Minimum zoom level
        }
    }
    
    #[allow(unused)]
    pub fn get_zoom_level(&self) -> f32 {
        self.zoom_level
    }
    
    #[allow(unused)]
    pub fn reset_zoom(&mut self) {
        self.zoom_level = 1.0;
    }
    
    // Check if the image is currently cleared/empty
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.texture.width() == 1.0 && self.texture.height() == 1.0
    }
    
    // Check if collision should be performed (not empty)
    #[allow(unused)]
    pub fn is_collidable(&self) -> bool {
        !self.is_empty()
    }
    
    // Public method for setting a preloaded texture that accepts the tuple directly
    #[allow(unused)]
    pub fn set_preload(&mut self, preloaded: (Texture2D, Option<Vec<u8>>, String)) {
        let (texture, mask, filename) = preloaded;
        self.texture = texture;
        self.transparency_mask = mask;
        self.filename = filename;
    }

    /// Clears the image by setting it to a 1x1 transparent pixel
    #[allow(unused)]
    pub fn clear(&mut self) {
        // Create a 1x1 transparent pixel texture
        let empty_texture = Texture2D::from_rgba8(1, 1, &[0, 0, 0, 0]);
        let empty_mask = Some(vec![0]); // Single transparent pixel
        
        // Update the image object with this empty texture
        self.texture = empty_texture;
        self.transparency_mask = empty_mask;
        self.filename = "__empty__".to_string();
    }

    /// Method to set a new image
    #[allow(unused)]
    pub async fn set_image(&mut self, image_path: &str) {
        self.set_texture(image_path).await;
    }
}

async fn generate_mask(texture_path: &str, width: usize, height: usize) -> Option<Vec<u8>> {
    let image = load_image(texture_path).await.unwrap();
    let pixels = image.bytes; // Image pixels in RGBA8 format
    
    // Check if the image format has an alpha channel at all (RGBA)
    // If pixels length isn't divisible by 4, it's not RGBA format
    if pixels.len() != width * height * 4 {
        // No alpha channel, return None immediately
        return None;
    }

   
    let mut has_transparency = false;

    // First, scan to see if the image has any transparency at all
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4; // Each pixel is 4 bytes (RGBA)
            let alpha = pixels[idx + 3]; // Get alpha channel
            
            if alpha < 255 {
                has_transparency = true;
                break;
            }
        }
        if has_transparency {
            break;
        }
    }

    // If there's no transparency, return None
    if !has_transparency {
        return None;
    }
 // Only create the mask if we know the image has transparency
 let mut mask = vec![0; (width * height + 7) / 8]; // Create a bitmask with enough bytes
    // Otherwise, create the transparency mask
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4; // Each pixel is 4 bytes (RGBA)
            let alpha = pixels[idx + 3]; // Get alpha channel
            let mask_byte_idx = (y * width + x) / 8; // Find which byte this pixel belongs to
            let bit_offset = (y * width + x) % 8; // Find the bit offset inside the byte

            if alpha > 0 {
                // Set the bit to 1 (opaque pixel)
                mask[mask_byte_idx] |= 1 << (7 - bit_offset);
            } else {
                // Set the bit to 0 (transparent pixel)
                mask[mask_byte_idx] &= !(1 << (7 - bit_offset));
            }
        }
    }

    Some(mask)
}

pub async fn set_texture_main(texture_path: &str) -> (Texture2D, Option<Vec<u8>>) {
    let texture = load_texture(texture_path).await.unwrap();
    texture.set_filter(FilterMode::Linear);
    let tex_width = texture.width() as usize;
    let tex_height = texture.height() as usize;
    let transparency_mask = generate_mask(texture_path, tex_width, tex_height).await;
    return (texture, transparency_mask);
}

