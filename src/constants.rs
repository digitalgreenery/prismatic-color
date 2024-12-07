
use crate::Color;
use crate::ColorModel;

pub const TRANSPARENT: Color = Color {components: [0., 1., 0., 0.], color_type: ColorModel::SphericalHWBA,};
pub const WHITE: Color = Color::spherical_hwb(0., 1., 0.);
pub const BLACK: Color = Color::spherical_hwb(0., 0., 1.);
pub const GREY: Color = Color::spherical_hwb(0., 0.5, 0.5);

pub const RED: Color = Color::spherical_hwb(0. / 360., 0., 0.);
pub const SALMON: Color = Color::spherical_hwb(0. / 360., 0.5, 0.);
pub const MAROON: Color = Color::spherical_hwb(0. / 360., 0., 0.5);
pub const BURGUNDY: Color = Color::spherical_hwb(0. / 360., 0.25, 0.25);

pub const VERMILLION: Color = Color::spherical_hwb(15. / 360., 0., 0.);
pub const PEACH: Color = Color::spherical_hwb(15. / 360., 0.5, 0.);
pub const AUBURN: Color = Color::spherical_hwb(15. / 360., 0., 0.5);
pub const UMBER: Color = Color::spherical_hwb(15. / 360., 0.25, 0.25);

pub const ORANGE: Color = Color::spherical_hwb(30. / 360., 0., 0.);
pub const TAN: Color = Color::spherical_hwb(30. / 360., 0.5, 0.);
pub const BROWN: Color = Color::spherical_hwb(30. / 360., 0., 0.5);
pub const BEIGE: Color = Color::spherical_hwb(30. / 360., 0.25, 0.25);

pub const AMBER: Color = Color::spherical_hwb(45. / 360., 0., 0.);
pub const STRAW: Color = Color::spherical_hwb(45. / 360., 0.5, 0.);
pub const CARAMEL: Color = Color::spherical_hwb(45. / 360., 0., 0.5);
pub const SAFFRON: Color = Color::spherical_hwb(45. / 360., 0.25, 0.25);

pub const YELLOW: Color = Color::spherical_hwb(60. / 360., 0., 0.);
pub const LEMON: Color = Color::spherical_hwb(60. / 360., 0.5, 0.);
pub const DRAB: Color = Color::spherical_hwb(60. / 360., 0., 0.5);
pub const MUSTARD: Color = Color::spherical_hwb(60. / 360., 0.25, 0.25);

pub const BECQUEREL: Color = Color::spherical_hwb(75. / 360., 0., 0.);
pub const VIRELL: Color = Color::spherical_hwb(75. / 360., 0.5, 0.);
pub const OLIVE: Color = Color::spherical_hwb(75. / 360., 0., 0.5);
pub const PICKLE: Color = Color::spherical_hwb(75. / 360., 0.25, 0.25);

pub const CHARTREUSE: Color = Color::spherical_hwb(90. / 360., 0., 0.);
pub const VIRIDINE: Color = Color::spherical_hwb(90. / 360., 0.5, 0.);
pub const FERN: Color = Color::spherical_hwb(90. / 360., 0., 0.5);
pub const PERIDOT: Color = Color::spherical_hwb(90. / 360., 0.25, 0.25);

pub const LIME: Color = Color::spherical_hwb(105. / 360., 0., 0.);
pub const PALMETTO: Color = Color::spherical_hwb(105. / 360., 0.5, 0.);
pub const MOSS: Color = Color::spherical_hwb(105. / 360., 0., 0.5);
pub const PETRICHOR: Color = Color::spherical_hwb(105. / 360., 0.25, 0.25);

pub const GREEN: Color = Color::spherical_hwb(120. / 360., 0., 0.);
pub const WILLOW: Color = Color::spherical_hwb(120. / 360., 0.5, 0.);
pub const FOREST: Color = Color::spherical_hwb(120. / 360., 0., 0.5);
pub const CLOVER: Color = Color::spherical_hwb(120. / 360., 0.25, 0.25);

pub const EMERALD: Color = Color::spherical_hwb(135. / 360., 0., 0.);
pub const HONEYDEW: Color = Color::spherical_hwb(135. / 360., 0.5, 0.);
pub const ERIN: Color = Color::spherical_hwb(135. / 360., 0., 0.5);
pub const SAGE: Color = Color::spherical_hwb(135. / 360., 0.25, 0.25);

pub const MINT: Color = Color::spherical_hwb(150. / 360., 0., 0.);
pub const CELADON: Color = Color::spherical_hwb(150. / 360., 0.5, 0.);
pub const CONIFER: Color = Color::spherical_hwb(150. / 360., 0., 0.5);
pub const JADE: Color = Color::spherical_hwb(150. / 360., 0.25, 0.25);

pub const TURQUOISE: Color = Color::spherical_hwb(165. / 360., 0., 0.);
pub const SEAFOAM: Color = Color::spherical_hwb(165. / 360., 0.5, 0.);
pub const TEAL: Color = Color::spherical_hwb(165. / 360., 0., 0.5);
pub const VERDIGRIS: Color = Color::spherical_hwb(165. / 360., 0.25, 0.25);

pub const CYAN: Color = Color::spherical_hwb(180. / 360., 0., 0.);
pub const AQUA: Color = Color::spherical_hwb(180. / 360., 0.5, 0.);
pub const DELUGE: Color = Color::spherical_hwb(180. / 360., 0., 0.5);
pub const AGAVE: Color = Color::spherical_hwb(180. / 360., 0.25, 0.25);

pub const CAPRI: Color = Color::spherical_hwb(195. / 360., 0., 0.);
pub const CELESTE: Color = Color::spherical_hwb(195. / 360., 0.5, 0.);
pub const MARINE: Color = Color::spherical_hwb(195. / 360., 0., 0.5);
pub const AEGEAN: Color = Color::spherical_hwb(195. / 360., 0.25, 0.25);

pub const AZURE: Color = Color::spherical_hwb(210. / 360., 0., 0.);
pub const CORNFLOWER: Color = Color::spherical_hwb(210. / 360., 0.5, 0.);
pub const MIDNIGHT: Color = Color::spherical_hwb(210. / 360., 0., 0.5);
pub const SLATE: Color = Color::spherical_hwb(210. / 360., 0.25, 0.25);

pub const CERULEAN: Color = Color::spherical_hwb(225. / 360., 0., 0.);
pub const BONNET: Color = Color::spherical_hwb(225. / 360., 0.5, 0.);
pub const SAPPHIRE: Color = Color::spherical_hwb(225. / 360., 0., 0.5);
pub const HADAL: Color = Color::spherical_hwb(225. / 360., 0.25, 0.25);

pub const BLUE: Color = Color::spherical_hwb(240. / 360., 0., 0.);
pub const PERIWINKLE: Color = Color::spherical_hwb(240. / 360., 0.5, 0.);
pub const NAVY: Color = Color::spherical_hwb(240. / 360., 0., 0.5);
pub const DUSK: Color = Color::spherical_hwb(240. / 360., 0.25, 0.25);

pub const INDIGO: Color = Color::spherical_hwb(255. / 360., 0., 0.);
pub const HYACINTH: Color = Color::spherical_hwb(255. / 360., 0.5, 0.);
pub const SODALITE: Color = Color::spherical_hwb(255. / 360., 0., 0.5);
pub const CONCORD: Color = Color::spherical_hwb(255. / 360., 0.25, 0.25);

pub const VIOLET: Color = Color::spherical_hwb(270. / 360., 0., 0.);
pub const LAVENDER: Color = Color::spherical_hwb(270. / 360., 0.5, 0.);
pub const PRUNE: Color = Color::spherical_hwb(270. / 360., 0., 0.5);
pub const VERONICA: Color = Color::spherical_hwb(270. / 360., 0.25, 0.25);

pub const PURPLE: Color = Color::spherical_hwb(285. / 360., 0., 0.);
pub const LILAC: Color = Color::spherical_hwb(285. / 360., 0.5, 0.);
pub const AMETHYST: Color = Color::spherical_hwb(285. / 360., 0., 0.5);
pub const UBE: Color = Color::spherical_hwb(285. / 360., 0.25, 0.25);

pub const MAGENTA: Color = Color::spherical_hwb(300. / 360., 0., 0.);
pub const PHLOX: Color = Color::spherical_hwb(300. / 360., 0.5, 0.);
pub const AUBERGINE: Color = Color::spherical_hwb(300. / 360., 0., 0.5);
pub const MAUVE: Color = Color::spherical_hwb(300. / 360., 0.25, 0.25);

pub const FUSCHIA: Color = Color::spherical_hwb(315. / 360., 0., 0.);
pub const BUBBLEGUM: Color = Color::spherical_hwb(315. / 360., 0.5, 0.);
pub const PLUM: Color = Color::spherical_hwb(315. / 360., 0., 0.5);
pub const THISTLE: Color = Color::spherical_hwb(315. / 360., 0.25, 0.25);

pub const ROSE: Color = Color::spherical_hwb(330. / 360., 0., 0.);
pub const PINK: Color = Color::spherical_hwb(330. / 360., 0.5, 0.);
pub const AMARANTH: Color = Color::spherical_hwb(330. / 360., 0., 0.5);
pub const RASPBERRY: Color = Color::spherical_hwb(330. / 360., 0.25, 0.25);

pub const RUBY: Color = Color::spherical_hwb(345. / 360., 0., 0.);
pub const STRAWBERRY: Color = Color::spherical_hwb(345. / 360., 0.5, 0.);
pub const CRIMSON: Color = Color::spherical_hwb(345. / 360., 0., 0.5);
pub const CERISE: Color = Color::spherical_hwb(345. / 360., 0.25, 0.25);

pub const QUATERNARY_COLORS: [[Color; 4]; 25] = [
    [TRANSPARENT,WHITE,GREY,BLACK],
    [RED,SALMON,BURGUNDY,MAROON],
    [VERMILLION,PEACH,UMBER,AUBURN],
    [ORANGE,TAN,BEIGE,BROWN],
    [AMBER,STRAW,SAFFRON,CARAMEL],
    [YELLOW,LEMON,MUSTARD,DRAB],
    [BECQUEREL,VIRELL,PICKLE,OLIVE],
    [CHARTREUSE,VIRIDINE,PERIDOT,FERN],
    [LIME,PALMETTO,PETRICHOR,MOSS],
    [GREEN,WILLOW,CLOVER,FOREST],
    [EMERALD,HONEYDEW,SAGE,ERIN],
    [MINT,CELADON,JADE,CONIFER],
    [TURQUOISE,SEAFOAM,VERDIGRIS,TEAL],
    [CYAN,AQUA,AGAVE,DELUGE],
    [CAPRI,CELESTE,AEGEAN,MARINE],
    [AZURE,CORNFLOWER,SLATE,MIDNIGHT],
    [CERULEAN,BONNET,HADAL,SAPPHIRE],
    [BLUE,PERIWINKLE,DUSK,NAVY],
    [INDIGO,HYACINTH,CONCORD,SODALITE],
    [VIOLET,LAVENDER,VERONICA,PRUNE],
    [PURPLE,LILAC,UBE,AMETHYST],
    [MAGENTA,PHLOX,MAUVE,AUBERGINE],
    [FUSCHIA,BUBBLEGUM,THISTLE,PLUM],
    [ROSE,PINK,RASPBERRY,AMARANTH],
    [RUBY,STRAWBERRY,CERISE,CRIMSON],
];



#[cfg(test)]
mod tests {
    use crate::constants::QUATERNARY_COLORS;
    #[test]
    fn print_hex() {
        let hex_colors: Vec<Vec<String>> = QUATERNARY_COLORS.iter().
            map(|hue| hue.iter().
                map(|color| color.to_linear_rgb().to_hex()).collect()
            ).collect();
        print!("{:?}",hex_colors);
    }
}