use std::ffi::{CString, CStr};

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        };

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success); }
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
            };

            return Err(error.to_string_lossy().into_owned());
        };

        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Self, String> { Shader::from_source(source, gl::VERTEX_SHADER) }
    pub fn from_frag_source(source: &CStr) -> Result<Self, String> { Shader::from_source(source, gl::FRAGMENT_SHADER) }

    pub fn id(&self) -> gl::types::GLuint { self.id }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

pub struct ShaderProgram {
    id: gl::types::GLuint
}

impl ShaderProgram {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()); }
        }

        unsafe { gl::LinkProgram(id); }

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success); }
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len); }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    id, len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                )
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(id, shader.id()); }
        }

        Ok(ShaderProgram { id })
    }

    pub fn id(&self) -> gl::types::GLuint { self.id }

    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn get_attrib_location(&self, name: &str) -> Result<gl::types::GLint, gl::types::GLuint> {
        let cstr = CString::new(name).unwrap();
        let handle = unsafe { gl::GetAttribLocation(self.id, cstr.as_ptr()) };
        let success = unsafe { gl::GetError() };
        if success != gl::NO_ERROR { return Err(success); }
        if handle == -1 { return Err(0x0501); }
        Ok(handle)
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<gl::types::GLint, gl::types::GLuint> {
        let cstr = CString::new(name).unwrap();
        let handle = unsafe { gl::GetUniformLocation(self.id, cstr.as_ptr()) };
        let success = unsafe { gl::GetError() };
        if success != gl::NO_ERROR { return Err(success); }
        Ok(handle)
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}
