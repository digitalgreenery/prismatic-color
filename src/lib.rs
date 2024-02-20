//Digital Greenery
//Spherical RGB library

use std::{f64::consts::PI, collections::HashMap, sync::Mutex, fmt};
use num_traits::{AsPrimitive, Unsigned, PrimInt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color{
    components: [f64;4],//maybe make a fixed point library in the future
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
    // YUVA,

    //La*b*
    // LabHCLA,
    // LabA,

}
//Alpha could be used for specular color

impl Color {
    // Color ructor
    pub const fn rgb(red: f64, green: f64, blue: f64) -> Color {
        Color { components: [red,green,blue,1.], color_type: ColorType::RGBA}
    }
    pub const fn hcl(hue: f64, chroma: f64, luminance: f64) -> Color {
        Color { components: [hue,chroma,luminance,1.], color_type: ColorType::SphericalHCLA}
    }
    pub const fn hwb(hue: f64, white: f64, black: f64) -> Color {
        Color { components: [hue,white,black,1.], color_type: ColorType::SphericalHWBA}
    }
    pub const fn cubic_hwb(hue: f64, white: f64, black: f64) -> Color {
        Color { components: [hue,white,black,1.], color_type: ColorType::CubicHWBA}
    }
    pub const fn hsv(hue: f64, saturation: f64, value: f64) -> Color {
        Color { components: [hue,saturation,value,1.], color_type: ColorType::HSVA,}
    }

    pub fn set_alpha(&self, alpha: f64) -> Color{
        if  self.color_type == ColorType::CMYK ||
            self.color_type == ColorType::RGBW {
                return *self
        }
        let components = self.components;
        Color { components: [components[0],components[1],components[2],alpha], color_type: self.color_type}
    }

    pub fn to_tuple(&self) -> (f64,f64,f64,f64){
        self.components.into()
    }

    pub const fn to_array(&self) -> [f64;4]{
        self.components
    }

    pub  fn to_rgb(&self) -> Color{
        if self.color_type == ColorType::RGBA {return *self}
        let components: [f64;4] =
        match self.color_type {
           // ColorType::WI => spectral_color_to_rgb(self.components),
            ColorType::RGBA => self.components,
            ColorType::CMYA => cmy_to_rgb(self.components),
            ColorType::RGBW => rgbw_to_rgb(self.components),
            ColorType::CMYK => cmyk_to_rgb(self.components),
            ColorType::SphericalHCLA => hcl_to_rgb(self.components),
            ColorType::SphericalHWBA => spherical_hwb_to_rgb(self.components),
            ColorType::HSLA => hsl_to_rgb(self.components),
            ColorType::HSVA => hsv_to_rgb(self.components),
            ColorType::CubicHWBA => cubic_hwb_to_rgb(self.components),
            // ColorType::YUVA => yuv_to_rgb(self.components),
            // ColorType::LabHCLA => lch_to_rgb(self.components),
            // ColorType::LabA => lab_to_rgb(self.components),
        };
        Color { components, color_type: ColorType::RGBA}
    }

     fn to_color(&self,color_type: ColorType, rgb_to_color: fn([f64;4])->[f64;4]) -> Color{
        if self.color_type == color_type {return *self}
        let components: [f64;4] = rgb_to_color(self.to_rgb().components);
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

    pub fn to_cmy(&self) -> Color{
        self.to_color(ColorType::CMYA, rgb_to_cmy)
    }

    pub fn to_rgbw(&self) -> Color{
        self.to_color(ColorType::RGBW, rgb_to_rgbw)
    }



    fn to_integers<T>(&self, scale: Option<T>) -> [T;4]
        where
            T: Unsigned + PrimInt + AsPrimitive<f64> + 'static,
            f64: AsPrimitive<T>,
    {
        let scale_value = scale.unwrap_or(T::max_value()).to_f64().unwrap();

        let a = (self.components[0] * scale_value).round().as_();
        let b = (self.components[1] * scale_value).round().as_();
        let c = (self.components[2] * scale_value).round().as_();
        let d = (self.components[3] * scale_value).round().as_();

        return [a,b,c,d]
    }  
    pub fn to_8bit(&self) -> u32{
        let (a,b,c,d) = self.to_integers::<u8>(Some(255)).into();
        let (a,b,c,d) = (a as u32, b as u32, c as u32, d as u32);
        return (a << 24) + (b << 16) + (c << 8) + d;
    }

    pub const TRANSPARENT:  Color = Color{components: [0.,0.,0.,0.], color_type: ColorType::SphericalHWBA };
    pub const WHITE:        Color = Color::hwb(0.,1.,0.);
    pub const BLACK:        Color = Color::hwb(0.,0.,1.);
    pub const GREY:         Color = Color::hwb(0.,0.5,0.5);

    pub const RED:          Color = Color::hwb(0./360.,0.,0.);
    pub const SALMON:       Color = Color::hwb(0./360.,0.5,0.);
    pub const MAROON:       Color = Color::hwb(0./360.,0.,0.5);
    pub const BURGUNDY:     Color = Color::hwb(0./360.,0.25,0.25);

    pub const VERMILLION:   Color = Color::hwb(15./360.,0.,0.);
    pub const PEACH:        Color = Color::hwb(15./360.,0.5,0.);
    pub const AUBURN:       Color = Color::hwb(15./360.,0.,0.5);
    pub const UMBER:        Color = Color::hwb(15./360.,0.25,0.25);

    pub const ORANGE:       Color = Color::hwb(30./360.,0.,0.);
    pub const TAN:          Color = Color::hwb(30./360.,0.5,0.);
    pub const BROWN:        Color = Color::hwb(30./360.,0.,0.5);
    pub const BEIGE:        Color = Color::hwb(30./360.,0.25,0.25);

    pub const AMBER:        Color = Color::hwb(45./360.,0.,0.);
    pub const STRAW:        Color = Color::hwb(45./360.,0.5,0.);
    pub const CARAMEL:      Color = Color::hwb(45./360.,0.,0.5);
    pub const SAFFRON:      Color = Color::hwb(45./360.,0.25,0.25);

    pub const YELLOW:       Color = Color::hwb(60./360.,0.,0.);
    pub const LEMON:        Color = Color::hwb(60./360.,0.5,0.);
    pub const DRAB:         Color = Color::hwb(60./360.,0.,0.5);
    pub const MUSTARD:      Color = Color::hwb(60./360.,0.25,0.25);

    pub const VIRELL:       Color = Color::hwb(75./360.,0.,0.);
    pub const BECQUEREL:    Color = Color::hwb(75./360.,0.5,0.);
    pub const OLIVE:        Color = Color::hwb(75./360.,0.,0.5);
    pub const PICKLE:       Color = Color::hwb(75./360.,0.25,0.25);

    pub const CHARTREUSE:   Color = Color::hwb(90./360.,0.,0.);
    pub const VIRIDINE:     Color = Color::hwb(90./360.,0.5,0.);
    pub const FERN:         Color = Color::hwb(90./360.,0.,0.5);
    pub const PERIDOT:      Color = Color::hwb(90./360.,0.25,0.25);
    
    pub const LIME:         Color = Color::hwb(105./360.,0.,0.);
    pub const PALMETTO:     Color = Color::hwb(105./360.,0.5,0.);
    pub const MOSS:         Color = Color::hwb(105./360.,0.,0.5);
    pub const PETRICHOR:    Color = Color::hwb(105./360.,0.25,0.25);

    pub const GREEN:        Color = Color::hwb(120./360.,0.,0.);
    pub const WILLOW:       Color = Color::hwb(120./360.,0.5,0.);
    pub const FOREST:       Color = Color::hwb(120./360.,0.,0.5);
    pub const SAGE:         Color = Color::hwb(120./360.,0.25,0.25);

    pub const EMERALD:      Color = Color::hwb(135./360.,0.,0.);
    pub const HONEYDEW:     Color = Color::hwb(135./360.,0.5,0.);
    pub const ERIN:         Color = Color::hwb(135./360.,0.,0.5);
    pub const CLOVER:       Color = Color::hwb(135./360.,0.25,0.25);

    pub const MINT:         Color = Color::hwb(150./360.,0.,0.);
    pub const CELADON:      Color = Color::hwb(150./360.,0.5,0.);
    pub const CONIFER:      Color = Color::hwb(150./360.,0.,0.5);
    pub const JADE:         Color = Color::hwb(150./360.,0.25,0.25);

    pub const TURQUOISE:    Color = Color::hwb(165./360.,0.,0.);
    pub const SEAFOAM:      Color = Color::hwb(165./360.,0.5,0.);
    pub const TEAL:         Color = Color::hwb(165./360.,0.,0.5);
    pub const VERDIGRIS:    Color = Color::hwb(165./360.,0.25,0.25);

    pub const CYAN:         Color = Color::hwb(180./360.,0.,0.);
    pub const AQUA:         Color = Color::hwb(180./360.,0.5,0.);
    pub const DELUGE:       Color = Color::hwb(180./360.,0.,0.5);
    pub const AGAVE:        Color = Color::hwb(180./360.,0.25,0.25);

    pub const CAPRI:        Color = Color::hwb(195./360.,0.,0.);
    pub const CELESTE:      Color = Color::hwb(195./360.,0.5,0.);
    pub const MARINE:       Color = Color::hwb(195./360.,0.,0.5);
    pub const AEGEAN:       Color = Color::hwb(195./360.,0.25,0.25);

    pub const AZURE:        Color = Color::hwb(210./360.,0.,0.);
    pub const CORNFLOWER:   Color = Color::hwb(210./360.,0.5,0.);
    pub const PRUSSIAN:     Color = Color::hwb(210./360.,0.,0.5);
    pub const DENIM:        Color = Color::hwb(210./360.,0.25,0.25);

    pub const CERULEAN:     Color = Color::hwb(225./360.,0.,0.);
    pub const BONNET:       Color = Color::hwb(225./360.,0.5,0.);
    pub const COBALT:       Color = Color::hwb(225./360.,0.,0.5);
    pub const HADAL:        Color = Color::hwb(225./360.,0.25,0.25);

    pub const BLUE:         Color = Color::hwb(240./360.,0.,0.);
    pub const PERIWINKLE:   Color = Color::hwb(240./360.,0.5,0.);
    pub const NAVY:         Color = Color::hwb(240./360.,0.,0.5);
    pub const SAPPHIRE:     Color = Color::hwb(240./360.,0.25,0.25);

    pub const INDIGO:       Color = Color::hwb(255./360.,0.,0.);
    pub const HYACINTH:     Color = Color::hwb(255./360.,0.5,0.);
    pub const SODALITE:     Color = Color::hwb(255./360.,0.,0.5);
    pub const CONCORD:      Color = Color::hwb(255./360.,0.25,0.25);

    pub const VIOLET:       Color = Color::hwb(270./360.,0.,0.);
    pub const LAVENDER:     Color = Color::hwb(270./360.,0.5,0.);
    pub const PRUNE:        Color = Color::hwb(270./360.,0.,0.5);
    pub const VERONICA:     Color = Color::hwb(270./360.,0.25,0.25);

    pub const PURPLE:       Color = Color::hwb(285./360.,0.,0.);
    pub const LILAC:        Color = Color::hwb(285./360.,0.5,0.);
    pub const AMETHYST:     Color = Color::hwb(285./360.,0.,0.5);
    pub const UBE:          Color = Color::hwb(285./360.,0.25,0.25);

    pub const MAGENTA:      Color = Color::hwb(300./360.,0.,0.);
    pub const PHLOX:        Color = Color::hwb(300./360.,0.5,0.);
    pub const AUBERGINE:    Color = Color::hwb(300./360.,0.,0.5);
    pub const MAUVE:        Color = Color::hwb(300./360.,0.25,0.25);

    pub const FUSCHIA:      Color = Color::hwb(315./360.,0.,0.);
    pub const BUBBLEGUM:    Color = Color::hwb(315./360.,0.5,0.);
    pub const PLUM:         Color = Color::hwb(315./360.,0.,0.5);
    pub const THISTLE:      Color = Color::hwb(315./360.,0.25,0.25);

    pub const ROSE:         Color = Color::hwb(330./360.,0.,0.);
    pub const PINK:         Color = Color::hwb(330./360.,0.5,0.);
    pub const AMARANTH:       Color = Color::hwb(330./360.,0.,0.5);
    pub const RASPBERRY:    Color = Color::hwb(330./360.,0.25,0.25);

    pub const RUBY:         Color = Color::hwb(345./360.,0.,0.);
    pub const STRAWBERRY:   Color = Color::hwb(345./360.,0.5,0.);
    pub const CRIMSON:      Color = Color::hwb(345./360.,0.,0.5);
    pub const CERISE:       Color = Color::hwb(345./360.,0.25,0.25);

    //pub fn fmt_hex()

}


//To RGBA
fn cmy_to_rgb(components: [f64; 4]) -> [f64; 4] {
    let (c,m, y, a) = components.into();
    [1.-c,1.-m,1.-y,a]
}

fn rgbw_to_rgb(components: [f64; 4]) -> [f64; 4] {
    let (r,g,b,w) = components.into();
    [r+w,g+w,b+w,0.]
}

fn cmyk_to_rgb(components: [f64; 4]) -> [f64; 4] {
    let (c,m,y,k) = components.into();
    [(1.-c)*(1.-k),(1.-m)*(1.-k),(1.-y)*(1.-k),0.]
}

fn hcl_to_rgb(hcl: [f64;4]) -> [f64;4] {
    let (hue,chroma,luminance,alpha) = hcl.into();

    //HCL approximate implementation of spherical RGB 
    //Spherical RGB has three sides: yellow, cyan, and magenta.
    //Phi is the angle towards the grey point for saturation.
    let hue = hue * 3.;
    let hue_angle: f64 = (PI / 2.0) * (hue % 1.0) * chroma + (PI / 4.0) * (1.0 - chroma);
    let phi: f64 = 1.95968918625 - 1.1 * (1.15074 - 0.7893882996 * chroma).sin();
    //Returns the xyz coordinate from the spherical coordinates
    let a: f64 = set_to_zero_if_small(luminance * hue_angle.cos() * phi.sin());
    let b: f64 = set_to_zero_if_small(luminance * hue_angle.sin() * phi.sin());
    let c: f64 = set_to_zero_if_small(luminance * phi.cos());

    let (r,g,b) =
    match  hue.floor() as u8 {
        0 => (a,b,c),
        1 => (c,a,b),
        _ => (b,c,a),
    };

    [r,g,b,alpha]
}

fn spherical_hwb_to_rgb(hwb: [f64;4]) -> [f64;4] {
    let (hue,white,black,alpha) = hwb.into();
    let hcl = [hue,1.-white,1.-black,alpha];
    hcl_to_rgb(hcl)
}

fn hsl_to_rgb(components: [f64; 4]) -> [f64; 4] {
    todo!()
}

fn hsv_to_rgb(hsv: [f64;4]) -> [f64;4] {
    let (hue, saturation, value, alpha) = hsv.into();
    let h = hue * 6.;
    let h_int = h as u8;
    let c = value * saturation;
    let max = value;
    let min = max - c;

    let (r,g,b) = match h_int {
        0 => (max,min+h*c,min),
        1 => (min-(h-2.)*c,max,min),
        2 => (min,max,min+(h-2.)*c),
        3 => (min,min-(h-4.)*c,max),
        4 => (min+(h-4.)*c,min,max),
        _ => (max,min,min-(h-6.)*c),

    };
    [r,g,b,alpha]
}

fn cubic_hwb_to_rgb(hwb: [f64;4]) -> [f64;4] {
    let (hue,white,black,alpha) = hwb.into();
    let saturation = 1. - (white/(1.-black));
    let value = 1. - black;
    let hsv = [hue,saturation,value,alpha];
    hsv_to_rgb(hsv)
}

// fn yuv_to_rgb(components: [f64; 4]) -> [f64; 4] {
//     todo!()
// }

// fn lch_to_rgb(components: [f64; 4]) -> [f64; 4] {
//     todo!()
// }

// fn lab_to_rgb(components: [f64; 4]) -> [f64; 4] {
//     todo!()
// }

//From RGBA
fn rgb_to_cmyk(components: [f64; 4]) -> [f64; 4] {
    let (r,g,b,_) = components.into();
    let black = [1.-r,1.-g,1.-b].min_value();
    [(1.-r-black)/(1.-black),(1.-g-black)/(1.-black),(1.-b-black)/(1.-black),black]
}

fn rgb_to_rgbw(components: [f64; 4]) -> [f64; 4] {
    let (r, g, b, _) = components.into();
    let w = r.min(g).min(b);
    [r - w, g - w, b - w, w]
}

fn rgb_to_cmy(components: [f64; 4]) -> [f64; 4] {
    let (r, g, b, a) = components.into();
    [1.-r, 1.-g, 1.-b, a]
}

fn rgb_to_hcl(rgb: [f64;4]) -> [f64;4] {

    let (r,g,b,alpha) = rgb.into();
    if set_to_zero_if_small(r.max(g).max(b))  == 0. {return [0.,0.,0.,alpha]}
    let (c,m,y,_) = rgb_to_cmy(rgb).into();
    let secondary = [y,c,m].index_of(c.max(m).max(y));

    let (a,b,c) =
    match secondary {
        0 => (r,g,b),
        1 => (g,b,r),
        _ => (b,r,g),
    };

    let luminance = (a*a+b*b+c*c).sqrt();
    //I need to fix these
    let phi = (c / luminance).acos();
    let hue_angle = b.atan2(a);
    let chroma = (((phi - 1.95968918625)/-1.1).asin() - 1.15074)/-0.7893882996;
    if chroma == 0. {
        let three: f64 = 3.0;
        let grey_point = 1./three.sqrt() * luminance;
        return [grey_point,grey_point,grey_point,alpha]}
    let hue = (((hue_angle - ((PI / 4.) * (1.-chroma)))/(PI / 2.) / chroma + secondary as f64) / 3.) % 1.;

    [hue,chroma,luminance,alpha]

}

fn rgb_to_spherical_hwb(rgb: [f64;4]) -> [f64;4] {
    let (h,c,l,a) = rgb_to_hcl(rgb).into();
    [h,1.-c,1.-l,a]
}

fn rgb_to_cubic_hwb(rgb: [f64;4]) -> [f64;4] {
    let (h,s,v,a) = rgb_to_hsv(rgb).into();
    let w = (1.-s) * v;
    let b = 1.-v;
    [h,w,b,a]
} 

fn rgb_to_hsl(rgb: [f64;4]) -> [f64;4] {
    let (r,g,b,alpha) = rgb.into();
    let min = [r,g,b].min_value();
    let max = [r,g,b].max_value();
    let chroma = max - min;
    let index = rgb.index_of(max);

    let lightness = (max + min) / 2.;

    let saturation = 
    if lightness < 0.5 { chroma / (max + min)} else {chroma / (2. - chroma)}; 

    let hue =
    if chroma == 0. {
        0.
    }
    else{
        match index {
            0 => ((g - b) / chroma + 0.) / 6.,
            1 => ((b - r) / chroma + 2.) / 6.,
            _ => ((r - g) / chroma + 4.) / 6.,
        }
    };

    [hue,saturation,lightness,alpha]
}

fn rgb_to_hsv(rgba: [f64;4]) -> [f64;4] {
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
        _ => g - r + 4.,
    };
    
    let h = (1. + h / 6.) % 1.;
    let s = delta/c_max;
    let v = c_max;

    return [h,s,v,a]
}

// Define a trait to add min and max methods to arrays of f64
trait ArrayExt {
    fn min_value(&self) -> f64;
    fn max_value(&self) -> f64;
    fn index_of(&self, value: f64) -> usize;
    // fn check_bounds(&self) -> [f64];
}

// Implement the trait for arrays of f64
impl ArrayExt for [f64] {
    fn min_value(&self) -> f64 {
        self.iter().fold(f64::INFINITY, |min: f64, x: &f64| if *x < min { *x } else { min })
    }

    fn max_value(&self) -> f64 {
        self.iter().fold(f64::NEG_INFINITY, |max, x| if *x > max { *x } else { max })
    }

    fn index_of(&self, value: f64) -> usize {
        self.iter().position(|&x| x == value).unwrap()
    }

    // fn check_bounds(&self) -> [f64] {
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

#[derive(Clone, Copy)]
pub enum NormalCurve {
    Linear,
    Power(f64),
    Quadratiic(f64,f64),
    Cubic(f64,f64,f64,f64),
}

struct ComposedMapping {
    curves: [Vec<(NormalCurve, f64, f64)>;4],
}


pub struct DefinedColor {
    color: Color,
    mapping_curve: ComposedMapping,
}

impl DefinedColor {
    pub fn new_linear(color: Color) -> DefinedColor {
        DefinedColor { color: color, mapping_curve: ComposedMapping { curves: [vec!((NormalCurve::Linear,1.,1.)),vec!((NormalCurve::Linear,1.,1.)),vec!((NormalCurve::Linear,1.,1.)),vec!((NormalCurve::Linear,1.,1.))] } }
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
//     pub r: f64,
//     pub g: f64,
//     pub b: f64,
// }

// impl Gamma {
//     // Color ructor
//     pub fn new(r: f64, g: f64, b: f64) -> Gamma {
//         Gamma { r, g, b }
//     }
//     pub fn from_tuple(tuple: (f64, f64, f64)) -> Gamma {
//         Gamma { r: (tuple.0), g: (tuple.1), b: (tuple.2) }
//     }
//     pub fn to_tuple(&self) ->  (f64, f64, f64){
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

 fn set_to_zero_if_small(value: f64) -> f64 {
    if value < 1e-7 {
        0.0
    }
    else{
        value
    }
}

pub fn tuple_lerp(tuple_a: (f64,f64,f64,f64), tuple_b: (f64,f64,f64,f64), percent: f64) -> (f64,f64,f64,f64){
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
        let num= 24;
        let angle = 360. / num as f64;
        for i in 0..num {
            // println!("The color {}",i);
            // let hsv = [i as f64 * angle/360.,1.,1.,1.];
            // let rgb:[f64; 4] = hsv_to_rgb(hsv);
            // let hsv_recovered = rgb_to_hsv(rgb);
            // println!("HSV: {:?}, RGB: {:?}, HSV Recovered: {:?}",hsv,rgb,hsv_recovered);
            // assert_eq!(hsv, hsv_recovered);

            // let hwb = [i as f64 * angle/360.,0.,0.,1.];//Won't hold true for values in grey
            // let rgb:[f64; 4] = cubic_hwb_to_rgb(hwb);
            // let hwb_recovered = rgb_to_cubic_hwb(rgb);
            // println!("HWB: {}, RGB: {}, HWB Recovered: {}",print_array(hwb),print_array(rgb),print_array(hwb_recovered));
            // // assert_eq!(hwb, hwb_recovered);

            let hcl = [i as f64 * angle/360. ,0.5,1.,1.];
            let rgb:[f64; 4] = hcl_to_rgb(hcl);
            let hcl_recovered = rgb_to_hcl(rgb);
            println!("HCL: {}, RGB: {}, HCL Recovered: {}",print_array(hcl),print_array(rgb),print_array(hcl_recovered));

            // let hsl = [i as f64 * angle/360.,1.,0.0,1.];
            // let rgb:[f64; 4] = [1.,0.5,0.,1.];//hsl_to_rgb(hsl);
            // let hsl_recovered = rgb_to_hsl(rgb);
            // println!("HSL: {:?}, RGB: {:?}, HSL Recovered: {:?}",hsl,rgb,hsl_recovered);
            // println!("");
        }
    }
    fn print_array(arr: [f64;4]) -> String {
        format!("[{:.2},{:.2},{:.2},{:.2}] ", arr[0],arr[1],arr[2],arr[3])
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
