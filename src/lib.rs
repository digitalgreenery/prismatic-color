//Digital Greenery
//Spherical RGB library

pub mod constants;
pub mod transformations;

use num_traits::{AsPrimitive, PrimInt, Unsigned};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    components: [f32; 4], //maybe make a fixed point library in the future
    color_type: ColorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    //Spectral Color
    // WBIS,

    //Component Representations
    RGBA,
    CMYA,

    //Keytone Representations
    RGBW,
    CMYK,

    //Spherical Representations
    SphericalHCLA,
    SphericalHWBA,
    // SphericalHSVA,

    //Cubic Representations
    //Circular Hue
    HSLA,
    CubicHSVA,
    CubicHWBA,
    //Square Hue
    YUVA,

    //La*b*
    // LabHCLA,
    // LabA,
}

impl Color {
    // Color constructor
    pub const fn rgb(red: f32, green: f32, blue: f32) -> Color {
        Color {
            components: [red, green, blue, 1.],
            color_type: ColorType::RGBA,
        }
    }
    pub const fn spherical_hcl(hue: f32, chroma: f32, luminance: f32) -> Color {
        Color {
            components: [hue, chroma, luminance, 1.],
            color_type: ColorType::SphericalHCLA,
        }
    }
    pub const fn spherical_hwb(hue: f32, white: f32, black: f32) -> Color {
        Color {
            components: [hue, white, black, 1.],
            color_type: ColorType::SphericalHWBA,
        }
    }
    pub const fn cubic_hwb(hue: f32, white: f32, black: f32) -> Color {
        Color {
            components: [hue, white, black, 1.],
            color_type: ColorType::CubicHWBA,
        }
    }
    pub const fn cubic_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        Color {
            components: [hue, saturation, value, 1.],
            color_type: ColorType::CubicHSVA,
        }
    }

    pub const fn from_array(components: [f32; 4], color_type: ColorType) -> Color{
        Color { components: components, color_type: color_type }
    }

    pub const fn from_tuple(components: (f32,f32,f32,f32), color_type: ColorType) -> Color{
        Color { components: [components.0,components.1,components.2,components.3], color_type: color_type }
    }   

    pub fn set_alpha(&self, alpha: f32) -> Color {
        if self.color_type == ColorType::CMYK || self.color_type == ColorType::RGBW {
            return *self;
        }
        let components = self.components;
        Color {
            components: [components[0], components[1], components[2], alpha],
            color_type: self.color_type,
        }
    }

    pub fn to_tuple(&self) -> (f32, f32, f32, f32) {
        self.components.into()
    }

    pub const fn to_array(&self) -> [f32; 4] {
        self.components
    }

    pub fn to_u8_array(&self) -> [u8; 4] {
        [
            (self.components[0] * 255.) as u8,
            (self.components[1] * 255.) as u8,
            (self.components[2] * 255.) as u8,
            (self.components[3] * 255.) as u8,
        ]
    }


    pub fn to_rgb(&self) -> Color {
        if self.color_type == ColorType::RGBA {
            return *self;
        }
        let components: [f32; 4] = match self.color_type {
            // ColorType::WI => spectral_color_to_rgb(self.components),
            ColorType::RGBA => self.components,
            ColorType::CMYA => cmy_to_rgb(self.components),
            ColorType::RGBW => rgbw_to_rgb(self.components),
            ColorType::CMYK => cmyk_to_rgb(self.components),
            ColorType::SphericalHCLA => spherical_hcl_to_rgb(self.components),
            ColorType::SphericalHWBA => spherical_hwb_to_rgb(self.components),
            // ColorType::SphericalHSVA => shperical_hsv_to_rgb(self.components),
            ColorType::HSLA => cubic_hcl_to_rgb(self.components),
            ColorType::CubicHSVA => cubic_hsv_to_rgb(self.components),
            ColorType::CubicHWBA => cubic_hwb_to_rgb(self.components),
            ColorType::YUVA => yuv_to_rgb(self.components),
            // ColorType::LabHCLA => lch_to_rgb(self.components),
            // ColorType::LabA => lab_to_rgb(self.components),
        };
        Color {
            components,
            color_type: ColorType::RGBA,
        }
    }

    fn to_color(&self, color_type: ColorType, rgb_to_color: fn([f32; 4]) -> [f32; 4]) -> Color {
        if self.color_type == color_type {
            return *self;
        }
        let components: [f32; 4] = rgb_to_color(self.to_rgb().components);
        Color {
            components,
            color_type: color_type,
        }
    }

    pub fn to_spherical_hcl(self) -> Color {
        self.to_color(ColorType::SphericalHCLA, rgb_to_spherical_hcl)
    }

    pub fn to_spherical_hwb(self) -> Color {
        self.to_color(ColorType::SphericalHWBA, rgb_to_spherical_hwb)
    }

    // pub fn to_spherical_hsv(self) -> Color {
    //     self.to_color(ColorType::SphericalHSVA, rgb_to_spherical_hsv)
    // }

    pub fn to_cubic_hwb(self) -> Color {
        self.to_color(ColorType::CubicHWBA, rgb_to_cubic_hwb)
    }

    pub fn to_hsl(self) -> Color {
        self.to_color(ColorType::HSLA, rgb_to_hsl)
    }

    pub fn to_cubic_hsv(self) -> Color {
        self.to_color(ColorType::CubicHSVA, rgb_to_cubic_hsv)
    }

    pub fn to_cmyk(self) -> Color {
        self.to_color(ColorType::CMYK, rgb_to_cmyk)
    }

    pub fn to_cmy(self) -> Color {
        self.to_color(ColorType::CMYA, rgb_to_cmy)
    }

    pub fn to_rgbw(self) -> Color {
        self.to_color(ColorType::RGBW, rgb_to_rgbw)
    }

    pub fn to_yuva(self) -> Color {
        self.to_color(ColorType::YUVA, rgb_to_yuv)
    }

    pub fn as_f32(self) -> (f32,f32,f32,f32){
        let (a,b,c,d) = self.to_array().into();
        return (a as f32,b as f32,c as f32, d as f32);
    }

    fn wrap_hue(&self) -> Color {
        let mut color = self.to_tuple();
        if color.0 >= 1. {
            color.0 = color.0 - 1.;
        }
        return Color{components: color.into(), color_type: self.color_type}
    }

    // Function to generate a gradient between two colors
    fn gradient_fn(start: &Color, end: &Color, steps: usize) -> Vec<Color> {
        let mut colors = Vec::with_capacity(steps);
        for i in 0..steps {
            let t = i as f32 / (steps as f32 - 1.0);  // t varies between 0.0 and 1.0
            
            let new_values = transformations::array_lerp(&start.to_array(), &end.to_array(), t);

            colors.push(Color {
                color_type: start.color_type, // Or handle the color type logic as needed
                components: new_values,
            });
        }

        colors
    } 

    fn gradient_hue(start: &Color, end: &Color, steps: usize) -> Vec<Color> {
        let start_hue =
        if start.components[1] == 0. && (start.color_type != ColorType::CubicHWBA ||  start.color_type != ColorType::SphericalHWBA) ||
        start.components[1] == start.components[2] && (start.color_type == ColorType::CubicHWBA ||  start.color_type == ColorType::SphericalHWBA) {
            end.components[0] 
        }
        else{
            start.components[0]
        };
        let end_hue =
        if end.components[0] < start.components[0] {
            end.components[0] + 1.
        }
        else{
            end.components[0]
        };
        let start_array = [start_hue,start.components[1],start.components[2],start.components[3]];
        let end_array = [end_hue,end.components[1],end.components[2],end.components[3]];
        let start = Color { components: start_array, color_type: start.color_type }; 
        let end = Color { components: end_array, color_type: end.color_type }; 
        return  Self::gradient_fn(&start, &end, steps).iter().map(|color| color.wrap_hue()).collect()
    }

    pub fn gradient(start: &Color, end: &Color, steps: usize) -> Vec<Color> {
        let color_type = start.color_type;
        let end = end.convert_color(color_type);
        return match color_type {
            //Spherical Representations
            ColorType::SphericalHCLA |
            ColorType::SphericalHWBA |
            //Cubic Representations
            ColorType::HSLA |
            ColorType::CubicHSVA |
            ColorType::CubicHWBA 
            => Self::gradient_hue(&start, &end, steps),
            _ => Self::gradient_fn(&start, &end, steps)
        
        }
    }    

    pub fn convert_colors(colors: Vec<Color>,color_type: ColorType) -> Vec<Color> {
        return colors.into_iter().map(|color| color.convert_color(color_type)).collect()
    }

    pub fn convert_color(&self, color_type: ColorType) -> Color {
        if self.color_type == color_type {
            return *self;
        }
        let color = self.to_rgb();
        return match color_type {
            ColorType::RGBA => color,
            ColorType::CMYA => color.to_cmy(),
            ColorType::RGBW => color.to_rgbw(),
            ColorType::CMYK => color.to_cmyk(),
            ColorType::SphericalHCLA => color.to_spherical_hcl(),
            ColorType::SphericalHWBA => color.to_spherical_hwb(),
            ColorType::HSLA => color.to_hsl(),
            ColorType::CubicHSVA => color.to_cubic_hsv(),
            ColorType::CubicHWBA => color.to_cubic_hwb(),
            ColorType::YUVA => color.to_yuva(),
        }
    }

    pub fn to_integers<T>(self, scale: Option<T>) -> [T; 4]
    where
        T: Unsigned + PrimInt + AsPrimitive<f32> + 'static,
        f32: AsPrimitive<T>,
    {
        let scale_value = scale.unwrap_or(T::max_value()).to_f32().unwrap();

        let a = (self.components[0] * scale_value).round().as_();
        let b = (self.components[1] * scale_value).round().as_();
        let c = (self.components[2] * scale_value).round().as_();
        let d = (self.components[3] * scale_value).round().as_();

        return [a, b, c, d];
    }
    
    pub fn to_alpha_8888_u32(self) -> u32 {
        let (a, b, c, d) = self.to_integers::<u8>(Some(255)).into();
        let (a, b, c, d) = (a as u32, b as u32, c as u32, d as u32);
        return (d << 24) + (a << 16) + (b << 8) + c;
    }

    pub fn to_argb_u32(self) -> u32 {
        return self.to_rgb().to_alpha_8888_u32();
    }

    pub fn to_hex(self) -> String {
        let rgb = self.to_rgb().to_argb_u32();
        format!("{:06X}", rgb & 0xFFFFFF)
    }

    pub fn to_alpha_hex(self) -> String {
        let argb = self.to_argb_u32();
        format!("{:08X}", argb)
    }

    pub fn to_linear_rgb(self) -> Color{
        transformations::DefinedColor::gamma(self.to_rgb(), 2.2).collapse_color()
    }

    // pub fn to_565_u16(self) -> u16 {
    //     let 
    // }

}

//To RGBA
fn cmy_to_rgb(components: [f32; 4]) -> [f32; 4] {
    let (c, m, y, a) = components.into();
    [1. - c, 1. - m, 1. - y, a]
}

fn rgbw_to_rgb(components: [f32; 4]) -> [f32; 4] {
    let (r, g, b, w) = components.into();
    [r + w, g + w, b + w, 0.]
}

fn cmyk_to_rgb(components: [f32; 4]) -> [f32; 4] {
    let (c, m, y, k) = components.into();
    [
        (1. - c) * (1. - k),
        (1. - m) * (1. - k),
        (1. - y) * (1. - k),
        0.,
    ]
}

fn spherical_hcl_to_rgb(hcl: [f32; 4]) -> [f32; 4] {
    let (hue, chroma, luminance, alpha) = hcl.into();
    if chroma == 0. {
        let three: f32 = 3.0;
        let grey_point = 1. / three.sqrt() * luminance;
        return [grey_point, grey_point, grey_point, alpha];
    }
    //HCL approximate implementation of spherical RGB
    //Spherical RGB has three sides: yellow, cyan, and magenta.
    //Phi is the angle towards the grey point for saturation.
    let hue = hue * 3.;
    let hue_angle: f32 = (PI / 2.0) * (hue % 1.0) * chroma + (PI / 4.0) * (1.0 - chroma);
    let phi: f32 = 1.95968918625 - 1.1 * (1.15074 - 0.7893882996 * chroma).sin();
    //Returns the xyz coordinate from the spherical coordinates
    let a: f32 = luminance * hue_angle.cos() * phi.sin();
    let b: f32 = luminance * hue_angle.sin() * phi.sin();
    let c: f32 = luminance * phi.cos();

    let (r, g, b) = match hue.floor() as u8 {
        0 => (a, b, c),
        1 => (c, a, b),
        _ => (b, c, a),
    };

    [r, g, b, alpha]
}

fn spherical_hwb_to_rgb(hwb: [f32; 4]) -> [f32; 4] {
    let (hue, white, black, alpha) = hwb.into();
    let hcl = [hue, 1. - white, 1. - black, alpha];
    spherical_hcl_to_rgb(hcl)
}

// fn spherical_hsv_to_rgb(hsv: [f32; 4]) -> [f32; 4] {
//     let (hue, white, black, alpha) = hsv.into();
//     let hcl = [hue, 1. - white, 1. - black, alpha];
//     spherical_hcl_to_rgb(hcl)
// }

fn cubic_hcl_to_rgb(components: [f32; 4]) -> [f32; 4] {
    let (hue, saturation, lightness, alpha) = components.into();
    let h = hue * 6.;
    let h_int = h as u8;
    let c = (1. - (2. * lightness - 1.).abs() ) * saturation;
    let min = lightness - 0.5 * c;
    let x = c * (1. - (h % 2. - 1.).abs()) + min;
    let c = c + min;

    let (r, g, b) = match h_int {
        0 => (c, x, min),
        1 => (x, c, min),
        2 => (min, c,x),
        3 => (min, x, c),
        4 => (x, min, c),
        _ => (c, min, x),
    };
    [r, g, b, alpha]
}

fn cubic_hsv_to_rgb(hsv: [f32; 4]) -> [f32; 4] {
    let (hue, saturation, value, alpha) = hsv.into();
    let h = hue * 6.;
    let h_int = h as u8;
    let c = value * saturation;
    let max = value;
    let min = max - c;

    let (r, g, b) = match h_int {
        0 => (max, min + h * c, min),
        1 => (min - (h - 2.) * c, max, min),
        2 => (min, max, min + (h - 2.) * c),
        3 => (min, min - (h - 4.) * c, max),
        4 => (min + (h - 4.) * c, min, max),
        _ => (max, min, min - (h - 6.) * c),
    };
    [r, g, b, alpha]
}

fn cubic_hwb_to_rgb(hwb: [f32; 4]) -> [f32; 4] {
    let (hue, white, black, alpha) = hwb.into();
    let saturation = 1. - (white / (1. - black));
    let value = 1. - black;
    let hsv = [hue, saturation, value, alpha];
    cubic_hsv_to_rgb(hsv)
}

fn yuv_to_rgb(yuva: [f32; 4]) -> [f32; 4] {
    let (y,u,v,a) = yuva.into();
    let r = y + v * 1.114;
    let g = y - 0.395 * u - 0.581 * v;
    let b = y + 2.033 * u;

    [r,g,b,a]
}

// fn lch_to_rgb(components: [f32; 4]) -> [f32; 4] {
//     todo!()
// }

// fn lab_to_rgb(components: [f32; 4]) -> [f32; 4] {
//     todo!()
// }

//From RGBA
fn rgb_to_cmyk(components: [f32; 4]) -> [f32; 4] {
    let (r, g, b, _) = components.into();
    let black = [1. - r, 1. - g, 1. - b].min_value();
    [
        (1. - r - black) / (1. - black),
        (1. - g - black) / (1. - black),
        (1. - b - black) / (1. - black),
        black,
    ]
}

fn rgb_to_rgbw(components: [f32; 4]) -> [f32; 4] {
    let (r, g, b, _) = components.into();
    let w = r.min(g).min(b);
    [r - w, g - w, b - w, w]
}

fn rgb_to_cmy(components: [f32; 4]) -> [f32; 4] {
    let (r, g, b, a) = components.into();
    [1. - r, 1. - g, 1. - b, a]
}

fn rgb_to_spherical_hcl(rgb: [f32; 4]) -> [f32; 4] {
    let (r, g, b, alpha) = rgb.into();
    if r.max(g).max(b) == 0. {
        return [0., 0., 0., alpha];
    }
    let (c, m, y, _) = rgb_to_cmy(rgb).into();
    let secondary = [y, c, m].index_of(c.max(m).max(y));

    let (a, b, c) = match secondary {
        0 => (r, g, b),
        1 => (g, b, r),
        _ => (b, r, g),
    };

    let luminance = (a * a + b * b + c * c).sqrt();
    //I need to fix these
    let phi = (c / luminance).acos();
    let hue_angle = b.atan2(a);
    let chroma = (((phi - 1.95968918625) / -1.1).asin() - 1.15074) / -0.7893882996;
    if chroma == 0. {
        return [0., 0., luminance, alpha];
    }
    let hue = (((hue_angle - ((PI / 4.) * (1. - chroma))) / (PI / 2.) / chroma + secondary as f32) / 3.) % 1.;

    [hue, chroma, luminance, alpha]
}

fn rgb_to_spherical_hwb(rgb: [f32; 4]) -> [f32; 4] {
    let (h, c, l, a) = rgb_to_spherical_hcl(rgb).into();
    [h, 1. - c, 1. - l, a]
}

// fn rgb_to_spherical_hsv(rgb: [f32; 4]) -> [f32; 4] {
//     let (h, c, l, a) = rgb_to_spherical_hcl(rgb).into();
//     [h, 1. - c, 1. - l, a] 
// }

fn rgb_to_cubic_hwb(rgb: [f32; 4]) -> [f32; 4] {
    let (h, s, v, a) = rgb_to_cubic_hsv(rgb).into();
    let w = (1. - s) * v;
    let b = 1. - v;
    [h, w, b, a]
}

fn rgb_to_hsl(rgb: [f32; 4]) -> [f32; 4] {
    let (r, g, b, alpha) = rgb.into();
    let min = [r, g, b].min_value();
    let max = [r, g, b].max_value();
    let chroma = max - min;
    let index = rgb.index_of(max);

    let lightness = (max + min) / 2.;

    let saturation = if lightness < 0.5 {
        chroma / (max + min)
    } else {
        chroma / (2. - chroma)
    };

    let hue = if chroma == 0. {
        0.
    } else {
        match index {
            0 => ((g - b) / chroma + 0.) / 6.,
            1 => ((b - r) / chroma + 2.) / 6.,
            _ => ((r - g) / chroma + 4.) / 6.,
        }
    };

    [hue, saturation, lightness, alpha]
}

fn rgb_to_cubic_hsv(rgba: [f32; 4]) -> [f32; 4] {
    let (r, g, b, a) = rgba.into();
    let rgb = [r, g, b];
    let c_max = rgb.max_value();
    let c_min = rgb.min_value();
    if c_min == c_max {
        return [0., 0., c_max, a];
    }
    let delta = c_max - c_min;
    let index = rgb.index_of(c_max);
    let (r, g, b) = (
        (c_max - r) / delta,
        (c_max - g) / delta,
        (c_max - b) / delta,
    );

    let h = match index {
        0 => b - g,
        1 => r - b + 2.,
        _ => g - r + 4.,
    };

    let h = (1. + h / 6.) % 1.;
    let s = delta / c_max;
    let v = c_max;

    return [h, s, v, a];
}

fn rgb_to_yuv(rgba: [f32; 4]) -> [f32; 4] {
    let (r,g,b,a) = rgba.into();
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let u = 0.492 * (b - y);
    let v = 0.877 * (r - y);

    [y,u,v,a]
}

// Define a trait to add min and max methods to arrays of f32
trait ArrayExt {
    fn min_value(&self) -> f32;
    fn max_value(&self) -> f32;
    fn index_of(&self, value: f32) -> usize;
    // fn check_bounds(&self) -> [f32];
}

// Implement the trait for arrays of f32
impl ArrayExt for [f32] {
    fn min_value(&self) -> f32 {
        self.iter().fold(
            f32::INFINITY,
            |min: f32, x: &f32| if *x < min { *x } else { min },
        )
    }

    fn max_value(&self) -> f32 {
        self.iter().fold(
            f32::NEG_INFINITY, 
            |max: f32, x: &f32| if *x > max { *x } else { max }
        )
    }

    fn index_of(&self, value: f32) -> usize {
        self.iter().position(|&x| x == value).unwrap()
    }

    // fn check_bounds(&self) -> [f32] {
    //     for &value in self {
    //         if value < 0.0 || value > 1.0 {
    //             panic!("Color components must be between 0 and 1");
    //         }
    //         else{
    //             *self
    //         }
    //     }
    // }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        color.components
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    fn from(color: Color) -> Self {
        let [r, g, b, a] = color.components;
        (r, g, b, a)
    }
}

impl From<Color> for (f64, f64, f64, f64) {
    fn from(color: Color) -> Self {
        let [r, g, b, a] = color.components;
        (r as f64, g as f64, b as f64, a as f64)
    }
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        let scale = 255.0;
        let [r, g, b, a] = color.components;
        [
            (r * scale).round() as u8,
            (g * scale).round() as u8,
            (b * scale).round() as u8,
            (a * scale).round() as u8,
        ]
    }
}

impl From<Color> for [u16; 4] {
    fn from(color: Color) -> Self {
        let scale = 65535.0;
        let [r, g, b, a] = color.components;
        [
            (r * scale).round() as u16,
            (g * scale).round() as u16,
            (b * scale).round() as u16,
            (a * scale).round() as u16,
        ]
    }
}

pub trait IntoColor {
    fn into_color(self, color_type: ColorType) -> Color;
}

impl IntoColor for [f32; 4] {
    fn into_color(self, color_type: ColorType) -> Color {
        Color {
            components: self,
            color_type,
        }
    }
}

impl IntoColor for (f32, f32, f32, f32) {
    fn into_color(self, color_type: ColorType) -> Color {
        Color {
            components: [self.0, self.1, self.2, self.3],
            color_type,
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_conversions() {
        let num = 24;
        let angle = 360. / num as f32;
        for i in 0..num {
            
            let hsl = [i as f32 * angle / 360., 0.6, 0.5, 0.5];
            let rgb: [f32; 4] = cubic_hcl_to_rgb(hsl);
            let hsl_recovered = rgb_to_hsl(rgb);
            println!(
                "HSL: {}, RGB: {}, HSL Recovered: {}",
                print_array(hsl),
                print_array(rgb),
                print_array(hsl_recovered)
            );
        }
    }
    fn print_array(arr: [f32; 4]) -> String {
        format!("[{:.2},{:.2},{:.2},{:.2}] ", arr[0], arr[1], arr[2], arr[3])
    }

    #[test]
    fn test_gradient() {
        let start = Color::spherical_hwb(0.0, 0.0, 0.0); // Black
        let end = Color::spherical_hwb(0.95, 0.0, 0.0);   // Red
        let steps = 10;

        let gradient = Color::gradient(&start, &end, steps);
        
        // Print the results for inspection
        for (i, color) in gradient.iter().enumerate() {
            println!("Step {}: {:?}", i, color);
        }

        // Assert the first and last colors to check correctness
        assert_eq!(gradient.first().unwrap(), &start);
        assert_eq!(gradient.last().unwrap(), &end);
    }

}
