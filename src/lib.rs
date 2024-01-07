//Digital Greenery
//Spherical RGB library

use std::{f32::consts::PI, collections::HashMap, sync::Mutex, fmt, ops::Index};
use num_traits::{AsPrimitive, Unsigned, PrimInt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color{
    components: [f32;4],
    color_type: ColorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorType {
    //Spectral Color
    // WI,

    //Component Representations
    RGBA,
    CMYA,

    //Keytone Representations
    RGBW,
    CMYK,

    //Spherical Representations
    SphericalHCLA,
    SphericalHWBA,

    //Cubic Representations
    //Cylindrical Hue
    HSLA,
    HSVA,
    CubicHWBA,
    //2D Hue
    YUVA,

    //La*b*
    LabHCLA,
    LabA,

}


impl Color {
    // Color ructor
    pub const fn rgb(red: f32, green: f32, blue: f32) -> Color {
        Color { components: [red,green,blue,1.], color_type: ColorType::RGBA}
    }
    pub const fn hcl(hue: f32, chroma: f32, luminance: f32) -> Color {
        Color { components: [hue,chroma,luminance,1.], color_type: ColorType::SphericalHCLA}
    }
    pub const fn hwb(hue: f32, white: f32, black: f32) -> Color {
        Color { components: [hue,white,black,1.], color_type: ColorType::SphericalHWBA}
    }
    pub const fn cubic_hwb(hue: f32, white: f32, black: f32) -> Color {
        Color { components: [hue,white,black,1.], color_type: ColorType::CubicHWBA}
    }
    pub const fn hsv(hue: f32, saturation: f32, value: f32) -> Color {
        Color { components: [hue,saturation,value,1.], color_type: ColorType::HSVA,}
    }

    pub fn set_alpha(&self, alpha: f32) -> Color{
        if  self.color_type == ColorType::CMYK ||
            self.color_type == ColorType::RGBW {
                return *self
        }
        let components = self.components;
        Color { components: [components[0],components[1],components[2],alpha], color_type: self.color_type}
    }

    pub const fn to_tuple(&self) -> (f32,f32,f32,f32){
        (self.components[0],self.components[1],self.components[2],self.components[3])
    }

    pub const fn to_array(&self) -> [f32;4]{
        self.components
    }

    pub  fn to_rgb(&self) -> Color{
        if self.color_type == ColorType::RGBA {return *self}
        let components: [f32;4] =
        match self.color_type {
           // ColorType::WI => spectral_color_to_rgb(self.components),
            ColorType::RGBA => self.components,
            ColorType::CMYA => cmy_to_rgb(self.components),
            ColorType::RGBW => rgbw_to_rgb(self.components),
            ColorType::CMYK => cmyk_to_rgb(self.components),
            ColorType::SphericalHCLA => spherical_hcl_to_rgb(self.components),
            ColorType::SphericalHWBA => spherical_hwb_to_rgb(self.components),
            ColorType::HSLA => hsl_to_rgb(self.components),
            ColorType::HSVA => hsv_to_rgb(self.components),
            ColorType::CubicHWBA => cubic_hwb_to_rgb(self.components),
            ColorType::YUVA => yuv_to_rgb(self.components),
            ColorType::LabHCLA => lch_to_rgb(self.components),
            ColorType::LabA => lab_to_rgb(self.components),
        };
        Color { components, color_type: ColorType::RGBA}
    }

     fn to_color(&self,color_type: ColorType, rgb_to_color: fn([f32;4])->[f32;4]) -> Color{
        if self.color_type == color_type {return *self}
        let components: [f32;4] = rgb_to_color(self.to_rgb().components);
        Color { components, color_type: color_type}
    }

    pub fn to_hcl(&self) -> Color{
        self.to_color(ColorType::SphericalHCLA, rgb_to_hcl)
    }

    pub fn to_spherical_hwb(&self) -> Color{
        self.to_color(ColorType::SphericalHWBA, rgb_to_spherical_hwb)
    }

    pub fn to_cubic_hwb(&self) -> Color{
        self.to_color(ColorType::CubicHWBA, rgb_to_cubic_hwb)
    }

    pub fn to_hsl(&self) -> Color{
        self.to_color(ColorType::HSLA, rgb_to_hsl)
    }

    pub fn to_hsv(&self) -> Color{
        self.to_color(ColorType::HSVA, rgb_to_hsv)
    }

    pub fn to_cmyk(&self) -> Color{
        self.to_color(ColorType::CMYK, rgb_to_cmyk)
    }

    // pub fn rgb_from_tuple(tuple (f32: r, f32, f32, f32)) -> Color {
    //     Color::RGB { red: r, green: g, blue: b, alpha: a}
    // }
    // pub fn color_lerp(color_a: Color, color_b: Color, percent: f32) -> Color {
    //     Color::from_tuple(tuple_lerp(color_a.to_tuple(), color_b.to_tuple(), percent))
    // }
    // pub fn to_integers<T>(&self, scale: Option<T>) -> (T, T, T)
    //     where
    //         T: Unsigned + PrimInt + AsPrimitive<f32> + 'static,
    //         f32: AsPrimitive<T>,
    // {
    //     let scale_value = scale.unwrap_or(T::max_value()).to_f32().unwrap();

    //     let r = (self.r * scale_value).round().as_();
    //     let g = (self.g * scale_value).round().as_();
    //     let b = (self.b * scale_value).round().as_();

    //     return (r,g,b)
    // }  
    // pub fn to_8bit_rgba(&self) -> u32{
    //     let (r,g,b) = self.to_integers::<u8>(Some(255));
    //     let (r,g,b) = (r as u32, g as u32, b as u32);
    //     return (r << 24) + (g << 16) + (b << 8) + 255;
    // }

    pub const TRANSPARENT: Color = Color{components: [0.,0.,0.,0.], color_type: ColorType::SphericalHWBA };
    pub const WHITE: Color = Color::hwb(0.,1.,0.);
    pub const BLACK: Color = Color::hwb(0.,0.,1.);
    pub const GREY: Color = Color::hwb(0.,0.5,0.5);

    pub const RED: Color = Color::hwb(0.,0.,0.);
    pub const SALMON: Color = Color::hwb(0.,0.5,0.);
    pub const MAROON: Color = Color::hwb(0.,0.,0.5);
    pub const BURGUNDY: Color = Color::hwb(0.,0.25,0.25);

    pub const VERMILLION: Color = Color::hwb(0.,0.,0.);
    pub const PEACH: Color = Color::hwb(0.,0.5,0.);
    pub const AUBURN: Color = Color::hwb(0.,0.,0.5);
    pub const UMBER: Color = Color::hwb(0.,0.25,0.25);

    pub const ORANGE: Color = Color::hwb(0.,0.,0.);
    pub const TAN: Color = Color::hwb(0.,0.5,0.);
    pub const BROWN: Color = Color::hwb(0.,0.,0.5);
    pub const BEIGE: Color = Color::hwb(0.,0.25,0.25);

    pub const AMBER: Color = Color::hwb(0.,0.,0.);
    pub const STRAW: Color = Color::hwb(0.,0.5,0.);
    pub const CARAMEL: Color = Color::hwb(0.,0.,0.5);
    pub const SAFFRON: Color = Color::hwb(0.,0.25,0.25);

    pub const YELLOW: Color = Color::hwb(0.,0.,0.);
    pub const LEMON: Color = Color::hwb(0.,0.5,0.);
    pub const DRAB: Color = Color::hwb(0.,0.,0.5);
    pub const MUSTARD: Color = Color::hwb(0.,0.25,0.25);

    pub const VIRELL: Color = Color::hwb(0.,0.,0.);
    pub const BECQUEREL: Color = Color::hwb(0.,0.5,0.);
    pub const OLIVE: Color = Color::hwb(0.,0.,0.5);
    pub const PICKLE: Color = Color::hwb(0.,0.25,0.25);

    pub const CHARTREUSE: Color = Color::hwb(0.,0.,0.);
    pub const VIRIDINE: Color = Color::hwb(0.,0.5,0.);
    pub const FERN: Color = Color::hwb(0.,0.,0.5);
    pub const PERIDOT: Color = Color::hwb(0.,0.25,0.25);
    
    pub const LIME: Color = Color::hwb(0.,0.,0.);
    pub const PALMETTO: Color = Color::hwb(0.,0.5,0.);
    pub const MOSS: Color = Color::hwb(0.,0.,0.5);
    pub const PETRICHOR: Color = Color::hwb(0.,0.25,0.25);

    pub const GREEN: Color = Color::hwb(0.,0.,0.);
    pub const WILLOW: Color = Color::hwb(0.,0.5,0.);
    pub const FOREST: Color = Color::hwb(0.,0.,0.5);
    pub const SAGE: Color = Color::hwb(0.,0.25,0.25);

    pub const EMERALD: Color = Color::hwb(0.,0.,0.);
    pub const HONEYDEW: Color = Color::hwb(0.,0.5,0.);
    pub const ERIN: Color = Color::hwb(0.,0.,0.5);
    pub const CLOVER: Color = Color::hwb(0.,0.25,0.25);

    pub const MINT: Color = Color::hwb(0.,0.,0.);
    pub const CELADON: Color = Color::hwb(0.,0.5,0.);
    pub const CONIFER: Color = Color::hwb(0.,0.,0.5);
    pub const JADE: Color = Color::hwb(0.,0.25,0.25);

    pub const TURQUOISE: Color = Color::hwb(0.,0.,0.);
    pub const SEAFOAM: Color = Color::hwb(0.,0.5,0.);
    pub const TEAL: Color = Color::hwb(0.,0.,0.5);
    pub const VERDIGRIS: Color = Color::hwb(0.,0.25,0.25);

    pub const CAPRI: Color = Color::hwb(0.,0.,0.);
    pub const CELESTE: Color = Color::hwb(0.,0.5,0.);
    pub const MARINE: Color = Color::hwb(0.,0.,0.5);
    pub const AEGEAN: Color = Color::hwb(0.,0.25,0.25);

    pub const AZURE: Color = Color::hwb(0.,0.,0.);
    pub const CORNFLOWER: Color = Color::hwb(0.,0.5,0.);
    pub const PRUSSIAN: Color = Color::hwb(0.,0.,0.5);
    pub const DENIM: Color = Color::hwb(0.,0.25,0.25);

    pub const CERULEAN: Color = Color::hwb(0.,0.,0.);
    pub const BONNET: Color = Color::hwb(0.,0.5,0.);
    pub const COBALT: Color = Color::hwb(0.,0.,0.5);
    pub const HADAL: Color = Color::hwb(0.,0.25,0.25);

    pub const BLUE: Color = Color::hwb(0.,0.,0.);
    pub const PERIWINKLE: Color = Color::hwb(0.,0.5,0.);
    pub const NAVY: Color = Color::hwb(0.,0.,0.5);
    pub const SAPPHIRE: Color = Color::hwb(0.,0.25,0.25);

    pub const INDIGO: Color = Color::hwb(0.,0.,0.);
    pub const HYACINTH: Color = Color::hwb(0.,0.5,0.);
    pub const SODALITE: Color = Color::hwb(0.,0.,0.5);
    pub const CONCORD: Color = Color::hwb(0.,0.25,0.25);

    pub const VIOLET: Color = Color::hwb(0.,0.,0.);
    pub const LAVENDER: Color = Color::hwb(0.,0.5,0.);
    pub const PRUNE: Color = Color::hwb(0.,0.,0.5);
    pub const VERONICA: Color = Color::hwb(0.,0.25,0.25);

    pub const PURPLE: Color = Color::hwb(0.,0.,0.);
    pub const LILAC: Color = Color::hwb(0.,0.5,0.);
    pub const AMETHYST: Color = Color::hwb(0.,0.,0.5);
    pub const UBE: Color = Color::hwb(0.,0.25,0.25);

    pub const MAGENTA: Color = Color::hwb(300./360.,0.,0.);
    pub const PHLOX: Color = Color::hwb(300./360.,0.5,0.);
    pub const AUBERGINE: Color = Color::hwb(300./360.,0.,0.5);
    pub const MAUVE: Color = Color::hwb(300./360.,0.25,0.25);

    pub const FUSCHIA: Color = Color::hwb(315./360.,0.,0.);
    pub const BUBBLEGUM: Color = Color::hwb(315./360.,0.5,0.);
    pub const PLUM: Color = Color::hwb(315./360.,0.,0.5);
    pub const THISTLE: Color = Color::hwb(315./360.,0.25,0.25);

    pub const ROSE: Color = Color::hwb(330./360.,0.,0.);
    pub const PINK: Color = Color::hwb(330./360.,0.5,0.);
    pub const CLARET: Color = Color::hwb(330./360.,0.,0.5);
    pub const RASPBERRY: Color = Color::hwb(330./360.,0.25,0.25);

    pub const RUBY: Color = Color::hwb(345./360.,0.,0.);
    pub const STRAWBERRY: Color = Color::hwb(345./360.,0.5,0.);
    pub const CRIMSON: Color = Color::hwb(345./360.,0.,0.5);
    pub const CERISE: Color = Color::hwb(345./360.,0.25,0.25);

    //pub fn fmt_hex()

}



fn cmy_to_rgb(components: [f32; 4]) -> [f32; 4] {
    let (c,m, y, a) = components.into();
    [1.-c,1.-m,1.-y,a]
}

fn rgbw_to_rgb(components: [f32; 4]) -> [f32; 4] {
    let (r,g,b,w) = components.into();
    [r+w,g+w,b+w,0.]
}

fn cmyk_to_rgb(components: [f32; 4]) -> [f32; 4] {
    let (c,m,y,k) = components.into();
    [(1.-c)*(1.-k),(1.-m)*(1.-k),(1.-y)*(1.-k),0.]
}

fn spherical_hcl_to_rgb(hcl: [f32;4]) -> [f32;4] {
    let (hue,chroma,luminance,alpha) = hcl.into();

    //Floats can't be used in a hash map so represent them as u32
    let key_hsv = (
        (hue * 10_000_000.0) as u32,
        (chroma * 10_000_000.0) as u32,
        (luminance * 10_000_000.0) as u32
    );

    // Define a static hash map for caching results
    lazy_static::lazy_static! {
        static ref CACHE: Mutex<HashMap<(u32, u32, u32), (f32, f32, f32)>> = Mutex::new(HashMap::new());
    }

    // Check if the result is already cached and return it if it exists
    if let Some((r,g,b)) = CACHE.lock().unwrap().get(&key_hsv) {
        return [*r,*g,*b,alpha];
    }

    //HSV approximate implementation of spherical RGB 
    //Spherical RGB has three sides: yellow, cyan, and magenta.
    //Phi is the angle towards the grey point for saturation.
    //Technically it's chroma and luminance, so this would actually be HCL, but more people are familiar with the term HSV, so whatever
    let hue_arc_length: f32 = 1.0 / 3.0;
    let hue_part: f32 = (PI / 2.0) * ((3.0 * hue) % 1.0) * chroma + (PI / 4.0) * (1.0 - chroma);
    let phi: f32 = 1.95968918625 - 1.1 * (1.15074 - 0.7893882996 * chroma).sin();
    //Returns the xyz coordinate from the spherical coordinates
    let a: f32 = set_to_zero_if_small(luminance * hue_part.cos() * phi.sin());
    let b: f32 = set_to_zero_if_small(luminance * hue_part.sin() * phi.sin());
    let c: f32 = set_to_zero_if_small(luminance * phi.cos());

    let result;

    if hue < hue_arc_length{              //Yellow Arc
        result = (a, b, c);
    } else if hue < 2.0 * hue_arc_length {//Cyan Arc
        result = (c, a, b);
    } else {                            //Magenta Arc
        result = (b, c, a);
    }

    // Cache the result
    CACHE.lock().unwrap().insert(key_hsv, result);

    let (r,g,b) = result;
    [r,g,b,alpha]
}

fn spherical_hwb_to_rgb(hwb: [f32;4]) -> [f32;4] {
    let (hue,white,black,alpha) = hwb.into();
    let hcl = [hue,1.-white,1.-black,alpha];
    spherical_hcl_to_rgb(hcl)
}

fn hsl_to_rgb(components: [f32; 4]) -> [f32; 4] {
    todo!()
}

fn hsv_to_rgb(hsv: [f32;4]) -> [f32;4] {
    let (hue, saturation, value, alpha) = hsv.into();
    let h = (hue * 6.).round() as u16 % 6;
    let c = value * saturation;
    let x = c * (1. - f32::abs((h as f32) % 2. - 1.));
    let m = value - c;

    let (r,g,b) = match h {
        0 => (c,x,0.),
        1 => (x,c,0.),
        2 => (0.,c,x),
        3 => (0.,x,c),
        4 => (x,0.,c),
        _ => (c,0.,x),

    };
    [r+m,g+m,b+m,alpha]
}

fn cubic_hwb_to_rgb(hwb: [f32;4]) -> [f32;4] {
    let (hue,white,black,alpha) = hwb.into();
    let saturation = 1. - (white/(1.-black));
    let value = 1. - black;
    let hsv = [hue,saturation,value,alpha];
    hsv_to_rgb(hsv)
}

fn yuv_to_rgb(components: [f32; 4]) -> [f32; 4] {
    todo!()
}

fn lch_to_rgb(components: [f32; 4]) -> [f32; 4] {
    todo!()
}

fn lab_to_rgb(components: [f32; 4]) -> [f32; 4] {
    todo!()
}

fn rgb_to_cmyk(components: [f32; 4]) -> [f32; 4] {
    // 1.-max
    todo!()
}

fn rgb_to_hcl(rgb: [f32;4]) -> [f32;4] {
    return [0.,0.,0.,0.]
}

fn rgb_to_spherical_hwb(rgb: [f32;4]) -> [f32;4] {
    let (h,c,l,a) = rgb_to_hcl(rgb).into();
    [h,1.-c,1.-l,a]
}


fn rgb_to_cubic_hwb(rgb: [f32;4]) -> [f32;4] {
    let (h,s,v,a) = rgb_to_hsv(rgb).into();
    let w = (1.-s) * v;
    let b = 1.-v;
    [h,w,b,a]
} 

fn rgb_to_hsl(rgb: [f32;4]) -> [f32;4] {
    return [0.,0.,0.,0.]
}

fn rgb_to_hsv(rgba: [f32;4]) -> [f32;4] {
    let (r,g,b,a) = rgba.into();
    let rgb = [r,g,b];
    let c_max = rgb.max_value(); 
    let c_min = rgb.min_value();
    if c_min == c_max{
        return [0.,0.,c_max,a]
    }
    let delta = c_max - c_min;
    let index = rgb.index_of(c_max);
    let (r, g, b) = ((c_max-r)/delta,(c_max-g)/delta,(c_max-b)/delta);
    
    let h = 
    match index {
        0 => b - g,
        1 => r - b + 2.,
        2 => g - r + 4.,
        _ => 0. //Should be unreachable
    };
    
    let h = h / 6. % 1.;
    let s = delta/c_max;
    let v = c_max;

    return [h,s,v,a]
}

// Define a trait to add min and max methods to arrays of f32
trait ArrayExt {
    fn min_value(&self) -> f32;
    fn max_value(&self) -> f32;
    fn index_of(&self, value: f32) -> usize;
}

// Implement the trait for arrays of f32
impl ArrayExt for [f32] {
    fn min_value(&self) -> f32 {
        self.iter().fold(f32::INFINITY, |min: f32, x: &f32| if *x < min { *x } else { min })
    }

    fn max_value(&self) -> f32 {
        self.iter().fold(f32::NEG_INFINITY, |max, x| if *x > max { *x } else { max })
    }

    fn index_of(&self, value: f32) -> usize {
        self.iter().position(|&x| x == value).unwrap()
    }

}


// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if f.alternate(){
//             write!(f,"{:06x}", (self.to_8bit_rgba() >> 8))
//         }
//         else{
//             write!(f, "Color (r: {}, g: {}, b: {})", self.r, self.g, self.b)
//         }
//     }
// }

// #[derive(Clone, Copy)]
// pub struct Gamma {
//     pub r: f32,
//     pub g: f32,
//     pub b: f32,
// }

// impl Gamma {
//     // Color ructor
//     pub fn new(r: f32, g: f32, b: f32) -> Gamma {
//         Gamma { r, g, b }
//     }
//     pub fn from_tuple(tuple: (f32, f32, f32)) -> Gamma {
//         Gamma { r: (tuple.0), g: (tuple.1), b: (tuple.2) }
//     }
//     pub fn to_tuple(&self) ->  (f32, f32, f32){
//         (self.r,self.g,self.b)
//     }
//     pub fn apply_gamma(&self, color: &Color) -> Color {
//         Color::new( color.r.powf(1.0/self.r),
//                     color.g.powf(1.0/self.g),
//                     color.b.powf(1.0/self.b),)
//     }
// }


// impl fmt::Display for Gamma {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Gamma (r: {}, g: {}, b: {})", self.r, self.g, self.b)
//     }
// }
 
// #[derive(Clone, Copy)]
// pub struct DefinedColor {
//     pub color: Color,
//     pub gamma: Gamma,
// }

// impl DefinedColor {
//     // Color ructor
//     pub fn new(color: Color, gamma: Gamma) -> DefinedColor {
//         DefinedColor { color, gamma }
//     }
//     pub fn to_color(&self) -> Color{
//         self.gamma.apply_gamma(&self.color)
//     }
// }

// impl fmt::Display for DefinedColor {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Defined Color (r: {}:{}, g: {}:{}, b: {}:{})", self.color.r, self.gamma.r, self.color.g, self.gamma.g, self.color.b, self.gamma.b)
//     }
// }

 fn set_to_zero_if_small(value: f32) -> f32 {
    if value < 1e-7 {
        0.0
    }
    else{
        value
    }
}

pub fn tuple_lerp(tuple_a: (f32,f32,f32,f32), tuple_b: (f32,f32,f32,f32), percent: f32) -> (f32,f32,f32,f32){
    (lerp(tuple_a.0,tuple_b.0,percent),
     lerp(tuple_a.1,tuple_b.1,percent),
     lerp(tuple_a.2,tuple_b.2,percent),
     lerp(tuple_a.3,tuple_b.3,percent),)
}

pub fn array_lerp<T>(array_a: &[T], array_b: &[T], percent: T) -> Vec<T>
where
    T: Copy + 
    std::ops::Add<Output = T> + 
    std::ops::Sub<Output = T> + 
    std::ops::Mul<Output = T>,
{
    array_a
        .iter()
        .zip(array_b.iter())
        .map(|(&a, &b)| lerp(a, b, percent))
        .collect()
}

pub fn lerp<T>(a: T, b: T, percent: T) -> T
where
    T: Copy + 
    std::ops::Add<Output = T> + 
    std::ops::Sub<Output = T> + 
    std::ops::Mul<Output = T>,
{
    
    a + (b - a) * percent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversions(){
        let hsv = [0.5,1.0,0.5,1.];
        let rgb:[f32; 4] = hsv_to_rgb(hsv);
        let hsv_recovered = rgb_to_hsv(rgb);
        println!("HSV: {:?}, RGB: {:?}, HSV Recovered: {:?}",hsv,rgb,hsv_recovered);
        assert_eq!(hsv, hsv_recovered);

        let hwb = [0.5,0.5,0.0,1.];//Won't hold true for values in grey
        let rgb:[f32; 4] = cubic_hwb_to_rgb(hwb);
        let hwb_recovered = rgb_to_cubic_hwb(rgb);
        println!("HWB: {:?}, RGB: {:?}, HWB Recovered: {:?}",hwb,rgb,hwb_recovered);
        assert_eq!(hwb, hwb_recovered);
    }

    #[test]
    fn test_lerp(){
        let array_a = vec![1., 2., 3., 4.];
        let array_b = vec![5., 6., 7., 8.];

        let result = array_lerp(&array_a, &array_b, 0.25);

        println!("{:?}", result);

        let array_a = vec![1., 0.5, 0.3, 1./7.];
        let array_b = vec![0., 0., 1., 0.8];

        let result = array_lerp(&array_a, &array_b, 0.25);

        println!("{:?}", result);
    }

}
