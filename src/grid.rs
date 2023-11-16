use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::camera::Camera;
use crate::shader::*;

const VERTEX_COLOR_STRING: &'static str = "ourColor";

pub struct Grid {
    pub tessellation_level: u32,
    pub vertices: [GLfloat; 48],
    pub indices: [u32;54],
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
    pub program: GLuint,
    pub color: [GLfloat; 3],
    pub camera : Camera

}

impl Grid {
    pub fn new() -> Self {

        let mut vertices = Grid::create_patch_vertices();
        let indices = Grid::create_indices_array();

        let (mut vao,mut vbo,mut ebo) = (0,0,0);
        let vs = Shader::new(VS_SRC, gl::VERTEX_SHADER);
        let fs = Shader::new(FS_SRC, gl::FRAGMENT_SHADER);
        let tcs = Shader::new(TCS_SRC,gl::TESS_CONTROL_SHADER);
        let tes = Shader::new(TES_SRC,gl::TESS_EVALUATION_SHADER);

        let program = link_program(vs.id, fs.id,tcs.id,tes.id);
        let color = [1.0,0.3,0.6];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1,&mut ebo);
        }

        let camera = Camera::new();
        vertices[7 * 3 + 2] = 0.5;

        let grid = Grid {tessellation_level : 1, vertices,indices, vao, vbo,ebo,program,color,camera};
        grid.init_grid();

        return grid;
    }

    fn create_patch_vertices() -> [GLfloat;48] {
        let n = 4;
        let stride = 1.0 / n as f32;

        let mut vertices : Vec<GLfloat> = vec![];

        for i in 0..n {
            for j in 0..n {
                vertices.push(-0.5 + stride * j as f32);
                vertices.push(-0.5 + stride * i as f32);
                vertices.push(0.0);
            }
        }
        let array = match vertices.try_into() {
            Ok(ba) => ba,
            Err(_) => panic!("Expected a Vec of length {} but it was different", 48),
        };
        return array;
    }

    fn create_indices_array() -> [u32;54] {
        let n = 3;

        let mut indices : Vec<u32> = vec![];

        for i in 0..3 {
            for j in 0..3 {
                //upper triangle
                indices.push(i* n +j);
                indices.push(i * n + j + 1);
                indices.push((i + 1) * n + j);

                //lower triangle
                indices.push(i * n + j + 1);
                indices.push((i + 1) * n + j + 1);
                indices.push((i + 1) * n + j);
            }
        }

        let array = match indices.try_into() {
            Ok(ba) => ba,
            Err(_) => panic!("Expected a Vec of length {} but it was {}", 54, -1),
        };
        return array;
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

            /*
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.indices[0]),
                gl::STATIC_DRAW,
            );
            */



            gl::VertexAttribPointer(
                0 as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );

            gl::EnableVertexAttribArray(0 as GLuint);

            gl::PatchParameteri(gl::PATCH_VERTICES,16);

        }
   }


    unsafe fn set_uniforms(&self) {
        unsafe {
            gl::UseProgram(self.program);
            let vertex_color_location = get_uniform_location(self.program,VERTEX_COLOR_STRING);

            let tessellation_level_location = get_uniform_location(self.program,"TessLevel");

            self.set_matrices();
            gl::Uniform3f(vertex_color_location,self.color[0],self.color[1],self.color[2]);
            gl::Uniform1ui(tessellation_level_location,self.tessellation_level);
        }
    }

    fn set_matrices(&self) {

        let view = self.camera.get_view();
        let projection = glam::Mat4::perspective_rh(
            70.0,
            SCREEN_WIDTH as f32/SCREEN_HEIGHT as f32,
            0.1,
            100.0);

        let mvp = projection * view;




        unsafe {
            let MVP_location = get_uniform_location(self.program,"MVP");
            let view_location = get_uniform_location(self.program,"ModelViewMatrix");
            let normal_Matrix = get_uniform_location(self.program,"NormalMatrix");
            gl::UniformMatrix4fv(MVP_location,1,gl::FALSE,mvp.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(view_location,1,gl::FALSE,view.to_cols_array().as_ptr());
            gl::UniformMatrix4fv(normal_Matrix,1,gl::FALSE,projection.to_cols_array().as_ptr());


        }

    }

    pub fn draw(&self) {
        unsafe {
            self.set_uniforms();
            gl::BindVertexArray(self.vao);
          //  gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT,ptr::null());
            gl::DrawArrays(gl::PATCHES,0,16);
            gl::BindVertexArray(0);
        }
    }
}

unsafe fn get_uniform_location(program: GLuint, uniform_name: &str) -> GLint {
    let cname = std::ffi::CString::new(uniform_name).expect("CString::new failed");

    unsafe {
      gl::GetUniformLocation(program,cname.as_ptr())
    }
}

pub fn link_program(vs: GLuint, fs: GLuint,tcs: GLuint, tes: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::AttachShader(program,tcs);
        gl::AttachShader(program,tes);
        gl::LinkProgram(program);

        gl::DetachShader(program, fs);
        gl::DetachShader(program, vs);
        gl::DetachShader(program, tcs);
        gl::DetachShader(program,tes);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteShader(tcs);
        gl::DeleteShader(tes);

        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

