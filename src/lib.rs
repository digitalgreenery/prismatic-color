//Digital Greenery
//Spherical RGB library

use std::{f32::consts::PI, collections::HashMap, sync::Mutex, fmt};
use num_traits::{AsPrimitive, ToPrimitive, Unsigned, PrimInt, FromPrimitive};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color{
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    // Color Constructor
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn from_tuple(tuple: (f32, f32, f32)) -> Color {
        Color { r: (tuple.0), g: (tuple.1), b: (tuple.2) }
    }
    pub fn to_tuple(&self) ->  (f32, f32, f32){
        (self.r,self.g,self.b)
    }
    pub fn color_lerp(color_a: Color, color_b: Color, percent: f32) -> Color {
        Color::from_tuple(tuple_lerp(color_a.to_tuple(), color_b.to_tuple(), percent))
    }
    pub fn to_integers<T>(&self, scale: Option<T>) -> (T, T, T)
        where
            T: Unsigned + PrimInt + AsPrimitive<f32> + 'static,
            f32: AsPrimitive<T>,
    {
        let scale_value = scale.unwrap_or(T::max_value()).to_f32().unwrap();

        let r = (self.r * scale_value).round().as_();
        let g = (self.g * scale_value).round().as_();
        let b = (self.b * scale_value).round().as_();

        return (r,g,b)
    }  
    pub fn to_8bit_rgba(&self) -> u32{
        let (r,g,b) = self.to_integers::<u8>(Some(255));
        let (r,g,b) = (r as u32, g as u32, b as u32);
        return (r << 24) + (g << 16) + (b << 8) + 255;
    }

    //pub fn fmt_hex()

}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate(){
            write!(f,"{:06x}", (self.to_8bit_rgba() >> 8))
        }
        else{
            write!(f, "Color (r: {}, g: {}, b: {})", self.r, self.g, self.b)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Gamma {
    r: f32,
    g: f32,
    b: f32,
}

impl Gamma {
    // Color Constructor
    pub fn new(r: f32, g: f32, b: f32) -> Gamma {
        Gamma { r, g, b }
    }
    pub fn from_tuple(tuple: (f32, f32, f32)) -> Gamma {
        Gamma { r: (tuple.0), g: (tuple.1), b: (tuple.2) }
    }
    pub fn to_tuple(&self) ->  (f32, f32, f32){
        (self.r,self.g,self.b)
    }
    pub fn apply_gamma(&self, color: &Color) -> Color {
        Color::new( color.r.powf(1.0/self.r),
                    color.g.powf(1.0/self.g),
                    color.b.powf(1.0/self.b),)
    }
}

impl fmt::Display for Gamma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gamma (r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

#[derive(Clone, Copy)]
pub struct DefinedColor {
    pub color: Color,
    pub gamma: Gamma,
}

impl DefinedColor {
    // Color Constructor
    pub fn new(color: Color, gamma: Gamma) -> DefinedColor {
        DefinedColor { color, gamma }
    }
    pub fn to_color(&self) -> Color{
        self.gamma.apply_gamma(&self.color)
    }
}

impl fmt::Display for DefinedColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Defined Color (r: {}:{}, g: {}:{}, b: {}:{})", self.color.r, self.gamma.r, self.color.g, self.gamma.g, self.color.b, self.gamma.b)
    }
}


pub fn spherical_hcl(hue: f32, chroma: f32, luminance: f32) -> Color{
    let hue = hue.min(1.0).max(0.0);
    let chroma = chroma.min(1.0).max(0.0);
    let luminance = luminance.min(1.0).max(0.0);

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
    if let Some(result) = CACHE.lock().unwrap().get(&key_hsv) {
        return Color::from_tuple(*result);
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


    Color::from_tuple(result)
}

fn set_to_zero_if_small(value: f32) -> f32 {
    if value < 1e-7 {
        0.0
    }
    else{
        value
    }
}

pub fn tuple_lerp(tuple_a: (f32,f32,f32), tuple_b: (f32,f32,f32), percent: f32) -> (f32,f32,f32){
    (lerp(tuple_a.0,tuple_b.0,percent),
     lerp(tuple_a.1,tuple_b.1,percent),
     lerp(tuple_a.2,tuple_b.2,percent),)
}

fn lerp(a: f32, b: f32, percent: f32) -> f32{
    a + (b - a) * percent
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
      println!("{}", spherical_hcl(0.0,1.0,1.0));
      println!("{}", spherical_hcl(-1.0,1.0,1.0));
      println!("{}", spherical_hcl(0.5,1.0,1.0));
      println!("{}", spherical_hcl(0.75,0.4,1.0));
      println!("{}", lerp(0.5,1.0,0.5));
    }

    #[test]
    fn tuple_test(){
        let (h,s,v) = (0.5,0.4,0.6);
        println!("{} is the same as {:?}", spherical_hcl(h,s,v), spherical_hcl(h,s,v).to_tuple());
    }

    #[test]
    fn orange(){
        let orange = spherical_hcl((30.0/360.0),1.0,1.0);
        println!("{} is also {:?}", orange, orange.to_integers::<u8>(Some(255_u8)) );
        
    }

    #[test]
    fn rainbow(){
        for hue in (0..360).step_by(30){
            let color = spherical_hcl((hue as f32/360.0),1.0,1.0);
            println!("{} is also {:?}", color, color.to_integers::<u8>(Some(255_u8)) );
        }
    }
    #[test]
    fn adjusted_rainbow(){
        for hue in 0..12{
            let color = Gamma::new(2.2, 2.2, 2.2).apply_gamma(& spherical_hcl((hue as f32 * 30.0) /360.0,1.0,1.0));
            println!("Color {}: {:08x}", hue, color.to_8bit_rgba() );
        }
    }

    #[test]

    fn defined_color(){
        println!("{:#}",DefinedColor::new(spherical_hcl(0.5, 0.75, 0.75),Gamma::new(2.0,2.4,2.0)));
        println!("{:#}",DefinedColor::new(spherical_hcl(0.5, 0.75, 0.75),Gamma::new(2.0,2.4,2.0)).to_color());
    }

    #[test]
    fn test_clone(){
        let orange = spherical_hcl((30.0/360.0),1.0,1.0);
        let orange_clone = orange.clone();
        assert_eq!(orange,orange_clone);
    }
}
