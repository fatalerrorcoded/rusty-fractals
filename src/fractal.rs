use std::ffi::CString;

use crate::shader::{Shader, ShaderProgram};

static SQUARE_VERTICES: &'static [f32]= &[
    -1.0, 1.0, 0.0,
    -1.0, -1.0, 0.0,
    1.0, -1.0, 0.0,
    1.0, 1.0, 0.0
];

static DRAW_ORDER: &'static [u16] = &[0, 1, 2, 0, 2, 3];

pub struct Fractal {
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

        Ok(Fractal { program })
    }

    pub fn draw(&self, mvp_matrix: &[f32]) {
        println!("{:?}", mvp_matrix);
        self.program.set_used();
        let position_handle = unsafe { gl::GetAttribLocation(self.program.id(), CString::new("vPosition").unwrap().as_ptr()) };
        let matrix_handle = unsafe { gl::GetUniformLocation(self.program.id(), CString::new("uMVPMatrix").unwrap().as_ptr()) };
        
        unsafe {
            gl::UniformMatrix4fv(matrix_handle, 1, false as gl::types::GLboolean, mvp_matrix.as_ptr());

            gl::EnableVertexAttribArray(position_handle as u32);
            gl::VertexAttribPointer(
                position_handle as u32, 3,
                gl::FLOAT, gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                SQUARE_VERTICES.as_ptr() as *const gl::types::GLvoid
            );

            gl::DrawElements(
                gl::TRIANGLES, DRAW_ORDER.len() as i32,
                gl::UNSIGNED_SHORT, DRAW_ORDER.as_ptr() as *const gl::types::GLvoid
            );

            gl::DisableVertexAttribArray(position_handle as u32);
        }
    }
}
