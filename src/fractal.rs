use std::ffi::CString;

use crate::shader::{Shader, ShaderProgram};

static DRAW_ORDER: &'static [u16] = &[0, 1, 2, 0, 2, 3];

pub struct Fractal {
    vbo: gl::types::GLuint,
    program: ShaderProgram,
}

impl Fractal {
    pub fn new() -> Result<Self, String> {
        let vertex_shader = Shader::from_vert_source(
            &CString::new(include_str!("../assets/vertex.glsl")).unwrap()
        )?;

        let fragment_shader = Shader::from_frag_source(
            &CString::new(include_str!("../assets/fragment.glsl")).unwrap()
        )?;

        let program = ShaderProgram::from_shaders(&[vertex_shader, fragment_shader])?;
        program.set_used();

        let square_vertices: Vec<f32> = vec![
            -1.0, 1.0, 0.0,
            -1.0, -1.0, 0.0,
            1.0, -1.0, 0.0,
            1.0, 1.0, 0.0
        ];

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (square_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                square_vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        /*let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, 3, gl::FLOAT, gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null()
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }*/

        Ok(Fractal { vbo, program })
    }

    pub fn draw(&self, mvp_matrix: &[f32]) {
        self.program.set_used();
        let position_handle = unsafe { gl::GetAttribLocation(self.program.id(), CString::new("vPosition").unwrap().as_ptr()) };
        let matrix_handle = unsafe { gl::GetUniformLocation(self.program.id(), CString::new("uMVPMatrix").unwrap().as_ptr()) };
        
        unsafe {
            gl::UniformMatrix4fv(matrix_handle, 1, false as gl::types::GLboolean, mvp_matrix.as_ptr());

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::EnableVertexAttribArray(position_handle as u32);
            gl::VertexAttribPointer(
                position_handle as u32, 3,
                gl::FLOAT, gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null()
            );

            gl::DrawElements(
                gl::TRIANGLES, DRAW_ORDER.len() as i32,
                gl::UNSIGNED_SHORT, DRAW_ORDER.as_ptr() as *const gl::types::GLvoid
            );

            gl::DisableVertexAttribArray(position_handle as u32);
        }
    }
}

impl Drop for Fractal {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
