use crate::{Color, IntoColor};
#[derive(Clone)]
enum NormalCurve {
    Linear,
    Power(f32),
    Quadratiic(f32, f32),
    Cubic(f32, f32, f32, f32),
    Composed(Vec<(NormalCurve, f32, f32)>),
}

impl NormalCurve {
    fn simple_arrary(&self) -> [NormalCurve; 4] {
        [self.clone(), self.clone(), self.clone(), self.clone()]
    }
}

trait Mapping {
    fn map_curve(self, curve: NormalCurve) -> f32;
    fn quadratic_mapping(self, x: f32, y: f32) -> f32;
    fn cubic_mapping(self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32;
    fn composed_mapping(self, curves: Vec<(NormalCurve, f32, f32)>) -> f32;
}
impl Mapping for f32 {
    fn map_curve(self, curve: NormalCurve) -> f32 {
        match curve {
            NormalCurve::Linear => self,
            NormalCurve::Power(a) => self.powf(a),
            NormalCurve::Quadratiic(x, y) => self.quadratic_mapping(x, y),
            NormalCurve::Cubic(x1, y1, x2, y2) => self.cubic_mapping(x1, y1, x2, y2),
            NormalCurve::Composed(curves) => self.composed_mapping(curves),
        }
    }
    fn quadratic_mapping(self, x1: f32, y1: f32) -> f32 {
        if x1 == 0.5 {
            return self * (self - 2. * y1 * (self - 1.))
        }
        else{
            let m = (x1 - (x1 * x1 - 2. * x1 * self + self).sqrt())/(2. * x1 - 1.);
            return 2. * (1. - m) * m * y1 + m.powi(2);
        }
    }
    fn cubic_mapping(self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        return 0.0;
    }

    fn composed_mapping(self, curves: Vec<(NormalCurve, f32, f32)>) -> f32 {
        return 0.0;
    }
}

pub struct DefinedColor {
    color: Color,
    mapping_curve: [NormalCurve; 4],
}
impl DefinedColor {
    pub fn linear(color: Color) -> DefinedColor {
        return DefinedColor {
            color: color,
            mapping_curve: NormalCurve::Linear.simple_arrary(),
        };
    }

    pub fn gamma(color: Color, power: f32) -> DefinedColor {
        return DefinedColor {
            color: color,
            mapping_curve: NormalCurve::Power(1. / power).simple_arrary(),
        };
    }

    pub fn component_gamma(color: Color, power: [f32; 4]) -> DefinedColor {
        return DefinedColor {
            color: color,
            mapping_curve: [
                NormalCurve::Power(1. / power[0]),
                NormalCurve::Power(1. / power[1]),
                NormalCurve::Power(1. / power[2]),
                NormalCurve::Power(1. / power[3]),
            ],
        };
    }

    pub fn quadratic(color: Color, x: f32, y: f32) -> DefinedColor {
        return DefinedColor {
            color: color,
            mapping_curve: [
                NormalCurve::Quadratiic(x, y),
                NormalCurve::Quadratiic(x, y),
                NormalCurve::Quadratiic(x, y),
                NormalCurve::Quadratiic(x, y),
            ],
        };
    }

    pub fn cubic(color: Color, x1: f32, y1: f32, x2: f32, y2: f32) -> DefinedColor {
        return DefinedColor {
            color: color,
            mapping_curve: [
                NormalCurve::Cubic(x1, y1, x2, y2),
                NormalCurve::Cubic(x1, y1, x2, y2),
                NormalCurve::Cubic(x1, y1, x2, y2),
                NormalCurve::Cubic(x1, y1, x2, y2),
            ],
        };
    }

    pub fn collapse_color(&self) -> Color {
        Color {
            components: [
                self.color.components[0].map_curve(self.mapping_curve[0].clone()),
                self.color.components[1].map_curve(self.mapping_curve[1].clone()),
                self.color.components[2].map_curve(self.mapping_curve[2].clone()),
                self.color.components[3].map_curve(self.mapping_curve[3].clone()),
            ],
            color_type: self.color.color_type,
        }
    }
}


pub fn tuple_lerp(
    tuple_a: (f32, f32, f32, f32),
    tuple_b: (f32, f32, f32, f32),
    percent: f32,
) -> (f32, f32, f32, f32) {
    (
        lerp(tuple_a.0, tuple_b.0, percent),
        lerp(tuple_a.1, tuple_b.1, percent),
        lerp(tuple_a.2, tuple_b.2, percent),
        lerp(tuple_a.3, tuple_b.3, percent),
    )
}

pub fn array_lerp<T, const N: usize>(array_a: &[T; N], array_b: &[T; N], percent: T) -> [T; N]
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
{
    let mut result = [array_a[0]; N];  // Create an array with the same type and size

    for i in 0..N {
        result[i] = lerp(array_a[i], array_b[i], percent);
    }

    result
}

pub fn color_lerp(a: Color, b: Color, percent: f32) -> Color {
    let lerp_array = array_lerp(&a.to_array(), &b.to_array(), percent);
    lerp_array.into_color(a.color_type)
} 

pub fn lerp<T>(a: T, b: T, percent: T) -> T
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
{
    a + (b - a) * percent
}

//Space Conversions
pub fn cylindrical_to_xyz(theta: f32, r: f32, z: f32) -> (f32, f32, f32) {
    let x = r * theta.cos();
    let y = r * theta.sin();
    (x, y, z)
}

pub fn xyz_to_cylindrical(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let r = (x.powi(2) + y.powi(2)).sqrt();
    let theta = y.atan2(x);
    (theta, r, z)
}

pub fn symmetric_to_xyz(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    (x, y + 0.5, z + 0.5)
}

pub fn xyz_to_symmetric(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    (x, y - 0.5, z - 0.5)
}

pub fn rotate_axes_clockwise(a: f32, b: f32, c: f32,) -> (f32, f32, f32) {
    (b,c,a)
}

pub fn rotate_axes_counterclockwise(a: f32, b: f32, c: f32,) -> (f32, f32, f32) {
    (c,a,b)
}

pub fn mirror_axes(a: f32, b: f32, c: f32,) -> (f32, f32, f32) {
    (a,c,b)
}

mod test{
    

    #[test]
    fn test_lerp() {
        let array_a = [1., 2., 3., 4.];
        let array_b = [5., 6., 7., 8.];

        let result = crate::transformations::array_lerp(&array_a, &array_b, 0.25);

        println!("{:?}", result);

        let array_a = [1., 0.5, 0.3, 1. / 7.];
        let array_b = [0., 0., 1., 0.8];

        let result = crate::transformations::array_lerp(&array_a, &array_b, 0.25);

        println!("{:?}", result);
    }
}
