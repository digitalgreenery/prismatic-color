
use crate::Color;
use crate::ColorType;

pub const TRANSPARENT: Color = Color {components: [0., 1., 0., 0.], color_type: ColorType::SphericalHWBA,};
pub const WHITE: Color = Color::hwb(0., 1., 0.);
pub const BLACK: Color = Color::hwb(0., 0., 1.);
pub const GREY: Color = Color::hwb(0., 0.5, 0.5);

pub const RED: Color = Color::hwb(0. / 360., 0., 0.);
pub const SALMON: Color = Color::hwb(0. / 360., 0.5, 0.);
pub const MAROON: Color = Color::hwb(0. / 360., 0., 0.5);
pub const BURGUNDY: Color = Color::hwb(0. / 360., 0.25, 0.25);

pub const VERMILLION: Color = Color::hwb(15. / 360., 0., 0.);
pub const PEACH: Color = Color::hwb(15. / 360., 0.5, 0.);
pub const AUBURN: Color = Color::hwb(15. / 360., 0., 0.5);
pub const UMBER: Color = Color::hwb(15. / 360., 0.25, 0.25);

pub const ORANGE: Color = Color::hwb(30. / 360., 0., 0.);
pub const TAN: Color = Color::hwb(30. / 360., 0.5, 0.);
pub const BROWN: Color = Color::hwb(30. / 360., 0., 0.5);
pub const BEIGE: Color = Color::hwb(30. / 360., 0.25, 0.25);

pub const AMBER: Color = Color::hwb(45. / 360., 0., 0.);
pub const STRAW: Color = Color::hwb(45. / 360., 0.5, 0.);
pub const CARAMEL: Color = Color::hwb(45. / 360., 0., 0.5);
pub const SAFFRON: Color = Color::hwb(45. / 360., 0.25, 0.25);

pub const YELLOW: Color = Color::hwb(60. / 360., 0., 0.);
pub const LEMON: Color = Color::hwb(60. / 360., 0.5, 0.);
pub const DRAB: Color = Color::hwb(60. / 360., 0., 0.5);
pub const MUSTARD: Color = Color::hwb(60. / 360., 0.25, 0.25);

pub const BECQUEREL: Color = Color::hwb(75. / 360., 0., 0.);
pub const VIRELL: Color = Color::hwb(75. / 360., 0.5, 0.);
pub const OLIVE: Color = Color::hwb(75. / 360., 0., 0.5);
pub const PICKLE: Color = Color::hwb(75. / 360., 0.25, 0.25);

pub const CHARTREUSE: Color = Color::hwb(90. / 360., 0., 0.);
pub const VIRIDINE: Color = Color::hwb(90. / 360., 0.5, 0.);
pub const FERN: Color = Color::hwb(90. / 360., 0., 0.5);
pub const PERIDOT: Color = Color::hwb(90. / 360., 0.25, 0.25);

pub const LIME: Color = Color::hwb(105. / 360., 0., 0.);
pub const PALMETTO: Color = Color::hwb(105. / 360., 0.5, 0.);
pub const MOSS: Color = Color::hwb(105. / 360., 0., 0.5);
pub const PETRICHOR: Color = Color::hwb(105. / 360., 0.25, 0.25);

pub const GREEN: Color = Color::hwb(120. / 360., 0., 0.);
pub const WILLOW: Color = Color::hwb(120. / 360., 0.5, 0.);
pub const FOREST: Color = Color::hwb(120. / 360., 0., 0.5);
pub const CLOVER: Color = Color::hwb(120. / 360., 0.25, 0.25);

pub const EMERALD: Color = Color::hwb(135. / 360., 0., 0.);
pub const HONEYDEW: Color = Color::hwb(135. / 360., 0.5, 0.);
pub const ERIN: Color = Color::hwb(135. / 360., 0., 0.5);
pub const SAGE: Color = Color::hwb(135. / 360., 0.25, 0.25);

pub const MINT: Color = Color::hwb(150. / 360., 0., 0.);
pub const CELADON: Color = Color::hwb(150. / 360., 0.5, 0.);
pub const CONIFER: Color = Color::hwb(150. / 360., 0., 0.5);
pub const JADE: Color = Color::hwb(150. / 360., 0.25, 0.25);

pub const TURQUOISE: Color = Color::hwb(165. / 360., 0., 0.);
pub const SEAFOAM: Color = Color::hwb(165. / 360., 0.5, 0.);
pub const TEAL: Color = Color::hwb(165. / 360., 0., 0.5);
pub const VERDIGRIS: Color = Color::hwb(165. / 360., 0.25, 0.25);

pub const CYAN: Color = Color::hwb(180. / 360., 0., 0.);
pub const AQUA: Color = Color::hwb(180. / 360., 0.5, 0.);
pub const DELUGE: Color = Color::hwb(180. / 360., 0., 0.5);
pub const AGAVE: Color = Color::hwb(180. / 360., 0.25, 0.25);

pub const CAPRI: Color = Color::hwb(195. / 360., 0., 0.);
pub const CELESTE: Color = Color::hwb(195. / 360., 0.5, 0.);
pub const MARINE: Color = Color::hwb(195. / 360., 0., 0.5);
pub const AEGEAN: Color = Color::hwb(195. / 360., 0.25, 0.25);

pub const AZURE: Color = Color::hwb(210. / 360., 0., 0.);
pub const CORNFLOWER: Color = Color::hwb(210. / 360., 0.5, 0.);
pub const MIDNIGHT: Color = Color::hwb(210. / 360., 0., 0.5);
pub const SLATE: Color = Color::hwb(210. / 360., 0.25, 0.25);

pub const CERULEAN: Color = Color::hwb(225. / 360., 0., 0.);
pub const BONNET: Color = Color::hwb(225. / 360., 0.5, 0.);
pub const SAPPHIRE: Color = Color::hwb(225. / 360., 0., 0.5);
pub const HADAL: Color = Color::hwb(225. / 360., 0.25, 0.25);

pub const BLUE: Color = Color::hwb(240. / 360., 0., 0.);
pub const PERIWINKLE: Color = Color::hwb(240. / 360., 0.5, 0.);
pub const NAVY: Color = Color::hwb(240. / 360., 0., 0.5);
pub const DUSK: Color = Color::hwb(240. / 360., 0.25, 0.25);

pub const INDIGO: Color = Color::hwb(255. / 360., 0., 0.);
pub const HYACINTH: Color = Color::hwb(255. / 360., 0.5, 0.);
pub const SODALITE: Color = Color::hwb(255. / 360., 0., 0.5);
pub const CONCORD: Color = Color::hwb(255. / 360., 0.25, 0.25);

pub const VIOLET: Color = Color::hwb(270. / 360., 0., 0.);
pub const LAVENDER: Color = Color::hwb(270. / 360., 0.5, 0.);
pub const PRUNE: Color = Color::hwb(270. / 360., 0., 0.5);
pub const VERONICA: Color = Color::hwb(270. / 360., 0.25, 0.25);

pub const PURPLE: Color = Color::hwb(285. / 360., 0., 0.);
pub const LILAC: Color = Color::hwb(285. / 360., 0.5, 0.);
pub const AMETHYST: Color = Color::hwb(285. / 360., 0., 0.5);
pub const UBE: Color = Color::hwb(285. / 360., 0.25, 0.25);

pub const MAGENTA: Color = Color::hwb(300. / 360., 0., 0.);
pub const PHLOX: Color = Color::hwb(300. / 360., 0.5, 0.);
pub const AUBERGINE: Color = Color::hwb(300. / 360., 0., 0.5);
pub const MAUVE: Color = Color::hwb(300. / 360., 0.25, 0.25);

pub const FUSCHIA: Color = Color::hwb(315. / 360., 0., 0.);
pub const BUBBLEGUM: Color = Color::hwb(315. / 360., 0.5, 0.);
pub const PLUM: Color = Color::hwb(315. / 360., 0., 0.5);
pub const THISTLE: Color = Color::hwb(315. / 360., 0.25, 0.25);

pub const ROSE: Color = Color::hwb(330. / 360., 0., 0.);
pub const PINK: Color = Color::hwb(330. / 360., 0.5, 0.);
pub const AMARANTH: Color = Color::hwb(330. / 360., 0., 0.5);
pub const RASPBERRY: Color = Color::hwb(330. / 360., 0.25, 0.25);

pub const RUBY: Color = Color::hwb(345. / 360., 0., 0.);
pub const STRAWBERRY: Color = Color::hwb(345. / 360., 0.5, 0.);
pub const CRIMSON: Color = Color::hwb(345. / 360., 0., 0.5);
pub const CERISE: Color = Color::hwb(345. / 360., 0.25, 0.25);

pub const QUATERNARY_COLORS: [[Color; 4]; 25] = [
    [TRANSPARENT,WHITE,BLACK,GREY],
    [RED,SALMON,MAROON,BURGUNDY],
    [VERMILLION,PEACH,AUBURN,UMBER],
    [ORANGE,TAN,BROWN,BEIGE],
    [AMBER,STRAW,CARAMEL,SAFFRON],
    [YELLOW,LEMON,DRAB,MUSTARD],
    [BECQUEREL,VIRELL,OLIVE,PICKLE],
    [CHARTREUSE,VIRIDINE,FERN,PERIDOT],
    [LIME,PALMETTO,MOSS,PETRICHOR],
    [GREEN,WILLOW,FOREST,CLOVER],
    [EMERALD,HONEYDEW,ERIN,SAGE],
    [MINT,CELADON,CONIFER,JADE],
    [TURQUOISE,SEAFOAM,TEAL,VERDIGRIS],
    [CYAN,AQUA,DELUGE,AGAVE],
    [CAPRI,CELESTE,MARINE,AEGEAN],
    [AZURE,CORNFLOWER,MIDNIGHT,SLATE],
    [CERULEAN,BONNET,SAPPHIRE,HADAL],
    [BLUE,PERIWINKLE,NAVY,DUSK],
    [INDIGO,HYACINTH,SODALITE,CONCORD],
    [VIOLET,LAVENDER,PRUNE,VERONICA],
    [PURPLE,LILAC,AMETHYST,UBE],
    [MAGENTA,PHLOX,AUBERGINE,MAUVE],
    [FUSCHIA,BUBBLEGUM,PLUM,THISTLE],
    [ROSE,PINK,AMARANTH,RASPBERRY],
    [RUBY,STRAWBERRY,CRIMSON,CERISE],
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