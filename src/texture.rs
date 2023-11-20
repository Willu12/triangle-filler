use std::path::Path;
use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::GLuint;
use image::EncodableLayout;

pub struct Texture {
    pub id : GLuint,

}

impl Texture {
    pub unsafe fn new() -> Texture {
        let mut texture = 0;
        gl::GenTextures(1,&mut texture);
        Self {id: 1}
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub unsafe fn load(&self, path: &Path){
        self.bind();

        let img = image::open(path).expect("failed_to_load_image").into_rgba8();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const _,
        );
        gl::ActiveTexture(gl::TEXTURE0);
    }


}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}