#[derive(Debug, Clone, Copy)]
pub struct ColorF32(f32);

impl ColorF32 {
    pub fn new(value: f32) -> Self {
        if value.is_nan() {
            return Self(0.0);
        }

        Self(value.clamp(0f32, 1f32))
    }
    pub fn get(&self) -> f32 {
        self.0
    }
}

impl From<f32> for ColorF32 {
    fn from(value: f32) -> Self {
        if value.is_nan() {
            return Self(0.0);
        }
        Self(value.clamp(0f32, 1f32))
    }
}

impl From<ColorF32> for f32 {
    fn from(value: ColorF32) -> Self {
        value.0
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Color<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl Color<u8> {
    pub fn new_f32<C: Into<ColorF32>>(r: C, g: C, b: C, a: C) -> Self {
        Color::<ColorF32>::new(r.into(), g.into(), b.into(), a.into()).into()
    }
}

impl From<Color<ColorF32>> for Color<u8> {
    fn from(value: Color<ColorF32>) -> Self {
        let to_u8 = |v: ColorF32| (v.get() * 255.0).round() as u8;

        Color {
            r: to_u8(value.r),
            g: to_u8(value.g),
            b: to_u8(value.b),
            a: to_u8(value.a),
        }
    }
}
