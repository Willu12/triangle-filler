use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use crate::triangle::Triangle;
use crate::triangle::*;
use std::mem;
use std::ptr;
use std::str;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::camera::Camera;
use crate::shader::*;

const VERTEX_COLOR_STRING: &'static str = "ourColor";

pub struct Grid {
    pub tessellation_level: u32,
    pub vertices: [GLfloat; 12],
    pub indices: [u32;6],
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
    pub program: GLuint,
    pub color: [GLfloat; 3],
    pub camera : Camera

}

impl Grid {
    pub fn new() -> Self {

        let vertices = [
            0.5,  0.5, 0.0,  // top right
            0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0,  // bottom left
            -0.5,  0.5, 0.0 ];
        let indices : [u32;6] =
            [0,1,3,
             1,2,3];

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        let vs = Shader::new(VS_SRC, gl::VERTEX_SHADER);
        let fs = Shader::new(FS_SRC, gl::FRAGMENT_SHADER);


        let program = link_program(vs.id, fs.id);
        let color = [1.0,0.3,0.6];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1,&mut ebo);
        }

        let camera = Camera::new();

        let mut grid = Grid {tessellation_level : 1, vertices,indices, vao, vbo,ebo,program,color,camera};
        grid.init_grid();

        return grid;
    }

   pub fn init_grid(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.vertices[0]),
                gl::STATIC_DRAW,
            );

            //
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.indices[0]),
                gl::STATIC_DRAW,
            );


            gl::VertexAttribPointer(
                0 as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );

            gl::EnableVertexAttribArray(0 as GLuint);
        }
   }

    pub fn draw(&self) {
        unsafe{

            let cname = std::ffi::CString::new(VERTEX_COLOR_STRING).expect("CString::new failed");
            let vertex_color_location = gl::GetUniformLocation(self.program,cname.as_ptr());
            let projection_location = gl::GetUniformLocation(self.program,std::ffi::CString::new("projection").expect("CString::new failed").as_ptr());
            let view_location = gl::GetUniformLocation(self.program,std::ffi::CString::new("view").expect("CString::new failed").as_ptr());
            let model_location = gl::GetUniformLocation(self.program,std::ffi::CString::new("model").expect("CString::new failed").as_ptr());

            let model = glam::Mat4::from_rotation_x(-10.0);
            let view = self.camera.get_view();//glam::Mat4::from_translation(glam::Vec3::new(0.0,0.0,-3.0));

            let projection = glam::Mat4::perspective_rh(
                70.0,
                SCREEN_WIDTH as f32/SCREEN_HEIGHT as f32,
                0.1,
                100.0);

            gl::UseProgram(self.program);
            gl::Uniform3f(vertex_color_location,self.color[0],self.color[1],self.color[2]);

            gl::UniformMatrix4fv(view_location,1,gl::FALSE,view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(projection_location,1,gl::FALSE,projection.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(model_location,1,gl::FALSE,model.to_cols_array().as_ptr());


            gl::BindVertexArray(self.vao);

            // Draw a triangle from the 3 vertices
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT,std::ptr::null());

            gl::BindVertexArray(0);

        }
    }



}

