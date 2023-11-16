use egui_sdl2_gl::egui;
use egui_sdl2_gl::gl::types::GLfloat;

pub fn add_sliders_to_egui(ui: &mut egui_sdl2_gl::egui::Ui, z_coords : &mut [GLfloat; 16]) {
    let n = 4;
    for i in 0..n {
        for j in 0..n {
            ui.add(egui::Slider::new(&mut z_coords[i * n + j],-1.0..=1.0));
        }
        ui.separator();
    }
}