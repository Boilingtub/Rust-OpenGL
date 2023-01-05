use gl;

pub struct ColorBuffer {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorBuffer {
    pub fn from_color(red : f32 , green : f32 , blue: f32 , alpha: f32) -> ColorBuffer {
        ColorBuffer {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    pub fn update_color(&mut self, red : f32, green: f32, blue: f32, alpha:f32) {
        self.r = red;
        self.g = green;
        self.b = blue;
        self.a = alpha;
    }

    pub fn set_used(&self, gl: &gl::Gl) {
        unsafe {
            gl.ClearColor(self.r , self.g , self.b, self.a);
        }
    }

    pub fn clear(&self, gl: &gl::Gl) {
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
