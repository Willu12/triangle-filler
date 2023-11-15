use egui_sdl2_gl::gl;
use egui_sdl2_gl::gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use crate::shader::*;

const VS_SRC: &'static str = "resources/shaders/vertexShader.glsl";
const FS_SRC: &'static str = "resources/shaders/fragmentShader.glsl";

pub type MyVertex = [GLfloat; 3];


pub struct Triangle {
    pub program: GLuint,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub vertices: [MyVertex; 3],
}



pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        gl::DetachShader(program, fs);
        gl::DetachShader(program, vs);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);

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

impl Triangle {
    pub fn new(vertices : [MyVertex;3]) -> Self {
        // Create Vertex Array Object
        let mut vao = 0;
        let mut vbo = 0;
        let vs = Shader::new(VS_SRC, gl::VERTEX_SHADER);
        let fs = Shader::new(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs.id, fs.id);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }
        Triangle { program, vao, vbo,vertices }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            let vertex_data =
                [(self.vertices[0] as [GLfloat;3]),([1.0,0.0,0.0] as [GLfloat;3]),
                (self.vertices[1] as [GLfloat;3]), ([0.0,1.0,0.0] as [GLfloat;3]),
                (self.vertices[2] as [GLfloat;3]), ([0.0,0.0,1.0] as [GLfloat;3])].concat();

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&vertex_data[0]),
                gl::STATIC_DRAW,
            );

            // Use shader program
            gl::UseProgram(self.program);

            gl::VertexAttribPointer(
                0 as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                6 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
           gl::EnableVertexAttribArray(0 as GLuint);

            // color triangle
            gl::VertexAttribPointer(
                1 as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                6 * mem::size_of::<GLfloat>() as GLsizei,
                (3 * mem::size_of::<GLfloat>() as GLsizei) as *const _,
            );
            gl::EnableVertexAttribArray(1 as GLuint);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

impl Drop for Triangle {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}