use std::ops::Mul;

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Alice Blue color { R:240, G:248, B:255, A:255 }
    pub const ALICE_BLUE: Color = Color::rgb(240, 248, 255);
    /// Antique White color { R:250, G:235, B:215, A:255 }
    pub const ANTIQUE_WHITE: Color = Color::rgb(250, 235, 215);
    /// Aqua color { R:0, G:255, B:255, A:255 }
    pub const AQUA: Color = Color::rgb(0, 255, 255);
    /// Aquamarine color { R:127, G:255, B:212, A:255 }
    pub const AQUAMARINE: Color = Color::rgb(127, 255, 212);
    /// Azure color { R:240, G:255, B:255, A:255 }
    pub const AZURE: Color = Color::rgb(240, 255, 255);
    /// Beige color { R:245, G:245, B:220, A:255 }
    pub const BEIGE: Color = Color::rgb(245, 245, 220);
    /// Bisque color { R:255, G:228, B:196, A:255 }
    pub const BISQUE: Color = Color::rgb(255, 228, 196);
    /// Black color { R:0, G:0, B:0, A:255 }
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    /// Blanched Almond color { R:255, G:235, B:205, A:255 }
    pub const BLANCHED_ALMOND: Color = Color::rgb(255, 235, 205);
    /// Blue color { R:0, G:0, B:255, A:255 }
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    /// Blue Violet color { R:138, G:43, B:226, A:255 }
    pub const BLUE_VIOLET: Color = Color::rgb(138, 43, 226);
    /// Brown color { R:165, G:42, B:42, A:255 }
    pub const BROWN: Color = Color::rgb(165, 42, 42);
    /// Burlywood color { R:222, G:184, B:135, A:255 }
    pub const BURLYWOOD: Color = Color::rgb(222, 184, 135);
    /// Cadet Blue color { R:95, G:158, B:160, A:255 }
    pub const CADET_BLUE: Color = Color::rgb(95, 158, 160);
    /// Chartreuse color { R:127, G:255, B:0, A:255 }
    pub const CHARTREUSE: Color = Color::rgb(127, 255, 0);
    /// Chocolate color { R:210, G:105, B:30, A:255 }
    pub const CHOCOLATE: Color = Color::rgb(210, 105, 30);
    /// Coral color { R:255, G:127, B:80, A:255 }
    pub const CORAL: Color = Color::rgb(255, 127, 80);
    /// Cornflower Blue color { R:100, G:149, B:237, A:255 }
    pub const CORNFLOWER_BLUE: Color = Color::rgb(100, 149, 237);
    /// Cornsilk color { R:255, G:248, B:220, A:255 }
    pub const CORNSILK: Color = Color::rgb(255, 248, 220);
    /// Crimson color { R:220, G:20, B:60, A:255 }
    pub const CRIMSON: Color = Color::rgb(220, 20, 60);
    /// Cyan color { R:0, G:255, B:255, A:255 }
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    /// Dark Blue color { R:0, G:0, B:139, A:255 }
    pub const DARK_BLUE: Color = Color::rgb(0, 0, 139);
    /// Dark Cyan color { R:0, G:139, B:139, A:255 }
    pub const DARK_CYAN: Color = Color::rgb(0, 139, 139);
    /// Dark Goldenrod color { R:184, G:134, B:11, A:255 }
    pub const DARK_GOLDENROD: Color = Color::rgb(184, 134, 11);
    /// Dark Gray color { R:169, G:169, B:169, A:255 }
    pub const DARK_GRAY: Color = Color::rgb(169, 169, 169);
    /// Dark Green color { R:0, G:100, B:0, A:255 }
    pub const DARK_GREEN: Color = Color::rgb(0, 100, 0);
    /// Dark Khaki color { R:189, G:183, B:107, A:255 }
    pub const DARK_KHAKI: Color = Color::rgb(189, 183, 107);
    /// Dark Magenta color { R:139, G:0, B:139, A:255 }
    pub const DARK_MAGENTA: Color = Color::rgb(139, 0, 139);
    /// Dark Olive Green color { R:85, G:107, B:47, A:255 }
    pub const DARK_OLIVE_GREEN: Color = Color::rgb(85, 107, 47);
    /// Dark Orange color { R:255, G:140, B:0, A:255 }
    pub const DARK_ORANGE: Color = Color::rgb(255, 140, 0);
    /// Dark Orchid color { R:153, G:50, B:204, A:255 }
    pub const DARK_ORCHID: Color = Color::rgb(153, 50, 204);
    /// Dark Red color { R:139, G:0, B:0, A:255 }
    pub const DARK_RED: Color = Color::rgb(139, 0, 0);
    /// Dark Salmon color { R:233, G:150, B:122, A:255 }
    pub const DARK_SALMON: Color = Color::rgb(233, 150, 122);
    /// Dark Sea Green color { R:143, G:188, B:143, A:255 }
    pub const DARK_SEA_GREEN: Color = Color::rgb(143, 188, 143);
    /// Dark Slate Blue color { R:72, G:61, B:139, A:255 }
    pub const DARK_SLATE_BLUE: Color = Color::rgb(72, 61, 139);
    /// Dark Slate Gray color { R:47, G:79, B:79, A:255 }
    pub const DARK_SLATE_GRAY: Color = Color::rgb(47, 79, 79);
    /// Dark Turquoise color { R:0, G:206, B:209, A:255 }
    pub const DARK_TURQUOISE: Color = Color::rgb(0, 206, 209);
    /// Dark Violet color { R:148, G:0, B:211, A:255 }
    pub const DARK_VIOLET: Color = Color::rgb(148, 0, 211);
    /// Deep Pink color { R:255, G:20, B:147, A:255 }
    pub const DEEP_PINK: Color = Color::rgb(255, 20, 147);
    /// Deep Sky Blue color { R:0, G:191, B:255, A:255 }
    pub const DEEP_SKY_BLUE: Color = Color::rgb(0, 191, 255);
    /// Dim Gray color { R:105, G:105, B:105, A:255 }
    pub const DIM_GRAY: Color = Color::rgb(105, 105, 105);
    /// Dodger Blue color { R:30, G:144, B:255, A:255 }
    pub const DODGER_BLUE: Color = Color::rgb(30, 144, 255);
    /// Firebrick color { R:178, G:34, B:34, A:255 }
    pub const FIREBRICK: Color = Color::rgb(178, 34, 34);
    /// Floral White color { R:255, G:250, B:240, A:255 }
    pub const FLORAL_WHITE: Color = Color::rgb(255, 250, 240);
    /// Forest Green color { R:34, G:139, B:34, A:255 }
    pub const FOREST_GREEN: Color = Color::rgb(34, 139, 34);
    /// Fuchsia color { R:255, G:0, B:255, A:255 }
    pub const FUCHSIA: Color = Color::rgb(255, 0, 255);
    /// Gainsboro color { R:220, G:220, B:220, A:255 }
    pub const GAINSBORO: Color = Color::rgb(220, 220, 220);
    /// Ghost White color { R:248, G:248, B:255, A:255 }
    pub const GHOST_WHITE: Color = Color::rgb(248, 248, 255);
    /// Gold color { R:255, G:215, B:0, A:255 }
    pub const GOLD: Color = Color::rgb(255, 215, 0);
    /// Goldenrod color { R:218, G:165, B:32, A:255 }
    pub const GOLDENROD: Color = Color::rgb(218, 165, 32);
    /// Gray color { R:190, G:190, B:190, A:255 }
    pub const GRAY: Color = Color::rgb(190, 190, 190);
    /// Web Gray color { R:128, G:128, B:128, A:255 }
    pub const WEB_GRAY: Color = Color::rgb(128, 128, 128);
    /// Green color { R:0, G:255, B:0, A:255 }
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    /// Web Green color { R:0, G:128, B:0, A:255 }
    pub const WEB_GREEN: Color = Color::rgb(0, 128, 0);
    /// Green Yellow color { R:173, G:255, B:47, A:255 }
    pub const GREEN_YELLOW: Color = Color::rgb(173, 255, 47);
    /// Honeydew color { R:240, G:255, B:240, A:255 }
    pub const HONEYDEW: Color = Color::rgb(240, 255, 240);
    /// Hot Pink color { R:255, G:105, B:180, A:255 }
    pub const HOT_PINK: Color = Color::rgb(255, 105, 180);
    /// Indian Red color { R:205, G:92, B:92, A:255 }
    pub const INDIAN_RED: Color = Color::rgb(205, 92, 92);
    /// Indigo color { R:75, G:0, B:130, A:255 }
    pub const INDIGO: Color = Color::rgb(75, 0, 130);
    /// Ivory color { R:255, G:255, B:240, A:255 }
    pub const IVORY: Color = Color::rgb(255, 255, 240);
    /// Khaki color { R:240, G:230, B:140, A:255 }
    pub const KHAKI: Color = Color::rgb(240, 230, 140);
    /// Lavender color { R:230, G:230, B:250, A:255 }
    pub const LAVENDER: Color = Color::rgb(230, 230, 250);
    /// Lavender Blush color { R:255, G:240, B:245, A:255 }
    pub const LAVENDER_BLUSH: Color = Color::rgb(255, 240, 245);
    /// Lawn Green color { R:124, G:252, B:0, A:255 }
    pub const LAWN_GREEN: Color = Color::rgb(124, 252, 0);
    /// Lemon Chiffon color { R:255, G:250, B:205, A:255 }
    pub const LEMON_CHIFFON: Color = Color::rgb(255, 250, 205);
    /// Light Blue color { R:173, G:216, B:230, A:255 }
    pub const LIGHT_BLUE: Color = Color::rgb(173, 216, 230);
    /// Light Coral color { R:240, G:128, B:128, A:255 }
    pub const LIGHT_CORAL: Color = Color::rgb(240, 128, 128);
    /// Light Cyan color { R:224, G:255, B:255, A:255 }
    pub const LIGHT_CYAN: Color = Color::rgb(224, 255, 255);
    /// Light Goldenrod color { R:250, G:250, B:210, A:255 }
    pub const LIGHT_GOLDENROD: Color = Color::rgb(250, 250, 210);
    /// Light Gray color { R:211, G:211, B:211, A:255 }
    pub const LIGHT_GRAY: Color = Color::rgb(211, 211, 211);
    /// Light Green color { R:144, G:238, B:144, A:255 }
    pub const LIGHT_GREEN: Color = Color::rgb(144, 238, 144);
    /// Light Pink color { R:255, G:182, B:193, A:255 }
    pub const LIGHT_PINK: Color = Color::rgb(255, 182, 193);
    /// Light Salmon color { R:255, G:160, B:122, A:255 }
    pub const LIGHT_SALMON: Color = Color::rgb(255, 160, 122);
    /// Light Sea Green color { R:32, G:178, B:170, A:255 }
    pub const LIGHT_SEA_GREEN: Color = Color::rgb(32, 178, 170);
    /// Light Sky Blue color { R:135, G:206, B:250, A:255 }
    pub const LIGHT_SKY_BLUE: Color = Color::rgb(135, 206, 250);
    /// Light Slate Gray color { R:119, G:136, B:153, A:255 }
    pub const LIGHT_SLATE_GRAY: Color = Color::rgb(119, 136, 153);
    /// Light Steel Blue color { R:176, G:196, B:222, A:255 }
    pub const LIGHT_STEEL_BLUE: Color = Color::rgb(176, 196, 222);
    /// Light Yellow color { R:255, G:255, B:224, A:255 }
    pub const LIGHT_YELLOW: Color = Color::rgb(255, 255, 224);
    /// Lime color { R:0, G:255, B:0, A:255 }
    pub const LIME: Color = Color::rgb(0, 255, 0);
    /// Lime Green color { R:50, G:205, B:50, A:255 }
    pub const LIME_GREEN: Color = Color::rgb(50, 205, 50);
    /// Linen color { R:250, G:240, B:230, A:255 }
    pub const LINEN: Color = Color::rgb(250, 240, 230);
    /// Magenta color { R:255, G:0, B:255, A:255 }
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    /// Maroon color { R:176, G:48, B:96, A:255 }
    pub const MAROON: Color = Color::rgb(176, 48, 96);
    /// Web Maroon color { R:128, G:0, B:0, A:255 }
    pub const WEB_MAROON: Color = Color::rgb(128, 0, 0);
    /// Medium Aquamarine color { R:102, G:205, B:170, A:255 }
    pub const MEDIUM_AQUAMARINE: Color = Color::rgb(102, 205, 170);
    /// Medium Blue color { R:0, G:0, B:205, A:255 }
    pub const MEDIUM_BLUE: Color = Color::rgb(0, 0, 205);
    /// Medium Orchid color { R:186, G:85, B:211, A:255 }
    pub const MEDIUM_ORCHID: Color = Color::rgb(186, 85, 211);
    /// Medium Purple color { R:147, G:112, B:219, A:255 }
    pub const MEDIUM_PURPLE: Color = Color::rgb(147, 112, 219);
    /// Medium Sea Green color { R:60, G:179, B:113, A:255 }
    pub const MEDIUM_SEA_GREEN: Color = Color::rgb(60, 179, 113);
    /// Medium Slate Blue color { R:123, G:104, B:238, A:255 }
    pub const MEDIUM_SLATE_BLUE: Color = Color::rgb(123, 104, 238);
    /// Medium Spring Green color { R:0, G:250, B:154, A:255 }
    pub const MEDIUM_SPRING_GREEN: Color = Color::rgb(0, 250, 154);
    /// Medium Turquoise color { R:72, G:209, B:204, A:255 }
    pub const MEDIUM_TURQUOISE: Color = Color::rgb(72, 209, 204);
    /// Medium Violet Red color { R:199, G:21, B:133, A:255 }
    pub const MEDIUM_VIOLET_RED: Color = Color::rgb(199, 21, 133);
    /// Midnight Blue color { R:25, G:25, B:112, A:255 }
    pub const MIDNIGHT_BLUE: Color = Color::rgb(25, 25, 112);
    /// Mint Cream color { R:245, G:255, B:250, A:255 }
    pub const MINT_CREAM: Color = Color::rgb(245, 255, 250);
    /// Misty Rose color { R:255, G:228, B:225, A:255 }
    pub const MISTY_ROSE: Color = Color::rgb(255, 228, 225);
    /// Moccasin color { R:255, G:228, B:181, A:255 }
    pub const MOCCASIN: Color = Color::rgb(255, 228, 181);
    /// Navajo White color { R:255, G:222, B:173, A:255 }
    pub const NAVAJO_WHITE: Color = Color::rgb(255, 222, 173);
    /// Navy Blue color { R:0, G:0, B:128, A:255 }
    pub const NAVY_BLUE: Color = Color::rgb(0, 0, 128);
    /// Old Lace color { R:253, G:245, B:230, A:255 }
    pub const OLD_LACE: Color = Color::rgb(253, 245, 230);
    /// Olive color { R:128, G:128, B:0, A:255 }
    pub const OLIVE: Color = Color::rgb(128, 128, 0);
    /// Olive Drab color { R:107, G:142, B:35, A:255 }
    pub const OLIVE_DRAB: Color = Color::rgb(107, 142, 35);
    /// Orange color { R:255, G:165, B:0, A:255 }
    pub const ORANGE: Color = Color::rgb(255, 165, 0);
    /// Orange Red color { R:255, G:69, B:0, A:255 }
    pub const ORANGE_RED: Color = Color::rgb(255, 69, 0);
    /// Orchid color { R:218, G:112, B:214, A:255 }
    pub const ORCHID: Color = Color::rgb(218, 112, 214);
    /// Pale Goldenrod color { R:238, G:232, B:170, A:255 }
    pub const PALE_GOLDENROD: Color = Color::rgb(238, 232, 170);
    /// Pale Green color { R:152, G:251, B:152, A:255 }
    pub const PALE_GREEN: Color = Color::rgb(152, 251, 152);
    /// Pale Turquoise color { R:175, G:238, B:238, A:255 }
    pub const PALE_TURQUOISE: Color = Color::rgb(175, 238, 238);
    /// Pale Violet Red color { R:219, G:112, B:147, A:255 }
    pub const PALE_VIOLET_RED: Color = Color::rgb(219, 112, 147);
    /// Papaya Whip color { R:255, G:239, B:213, A:255 }
    pub const PAPAYA_WHIP: Color = Color::rgb(255, 239, 213);
    /// Peach Puff color { R:255, G:218, B:185, A:255 }
    pub const PEACH_PUFF: Color = Color::rgb(255, 218, 185);
    /// Peru color { R:205, G:133, B:63, A:255 }
    pub const PERU: Color = Color::rgb(205, 133, 63);
    /// Pink color { R:255, G:192, B:203, A:255 }
    pub const PINK: Color = Color::rgb(255, 192, 203);
    /// Plum color { R:221, G:160, B:221, A:255 }
    pub const PLUM: Color = Color::rgb(221, 160, 221);
    /// Powder Blue color { R:176, G:224, B:230, A:255 }
    pub const POWDER_BLUE: Color = Color::rgb(176, 224, 230);
    /// Purple color { R:160, G:32, B:240, A:255 }
    pub const PURPLE: Color = Color::rgb(160, 32, 240);
    /// Web Purple color { R:128, G:0, B:128, A:255 }
    pub const WEB_PURPLE: Color = Color::rgb(128, 0, 128);
    /// Rebecca Purple color { R:102, G:51, B:153, A:255 }
    pub const REBECCA_PURPLE: Color = Color::rgb(102, 51, 153);
    /// Red color { R:255, G:0, B:0, A:255 }
    pub const RED: Color = Color::rgb(255, 0, 0);
    /// Rosy Brown color { R:188, G:143, B:143, A:255 }
    pub const ROSY_BROWN: Color = Color::rgb(188, 143, 143);
    /// Royal Blue color { R:65, G:105, B:225, A:255 }
    pub const ROYAL_BLUE: Color = Color::rgb(65, 105, 225);
    /// Saddle Brown color { R:139, G:69, B:19, A:255 }
    pub const SADDLE_BROWN: Color = Color::rgb(139, 69, 19);
    /// Salmon color { R:250, G:128, B:114, A:255 }
    pub const SALMON: Color = Color::rgb(250, 128, 114);
    /// Sandy Brown color { R:244, G:164, B:96, A:255 }
    pub const SANDY_BROWN: Color = Color::rgb(244, 164, 96);
    /// Sea Green color { R:46, G:139, B:87, A:255 }
    pub const SEA_GREEN: Color = Color::rgb(46, 139, 87);
    /// Seashell color { R:255, G:245, B:238, A:255 }
    pub const SEASHELL: Color = Color::rgb(255, 245, 238);
    /// Sienna color { R:160, G:82, B:45, A:255 }
    pub const SIENNA: Color = Color::rgb(160, 82, 45);
    /// Silver color { R:192, G:192, B:192, A:255 }
    pub const SILVER: Color = Color::rgb(192, 192, 192);
    /// Sky Blue color { R:135, G:206, B:235, A:255 }
    pub const SKY_BLUE: Color = Color::rgb(135, 206, 235);
    /// Slate Blue color { R:106, G:90, B:205, A:255 }
    pub const SLATE_BLUE: Color = Color::rgb(106, 90, 205);
    /// Slate Gray color { R:112, G:128, B:144, A:255 }
    pub const SLATE_GRAY: Color = Color::rgb(112, 128, 144);
    /// Snow color { R:255, G:250, B:250, A:255 }
    pub const SNOW: Color = Color::rgb(255, 250, 250);
    /// Spring Green color { R:0, G:255, B:127, A:255 }
    pub const SPRING_GREEN: Color = Color::rgb(0, 255, 127);
    /// Steel Blue color { R:70, G:130, B:180, A:255 }
    pub const STEEL_BLUE: Color = Color::rgb(70, 130, 180);
    /// Tan color { R:210, G:180, B:140, A:255 }
    pub const TAN: Color = Color::rgb(210, 180, 140);
    /// Teal color { R:0, G:128, B:128, A:255 }
    pub const TEAL: Color = Color::rgb(0, 128, 128);
    /// Thistle color { R:216, G:191, B:216, A:255 }
    pub const THISTLE: Color = Color::rgb(216, 191, 216);
    /// Tomato color { R:255, G:99, B:71, A:255 }
    pub const TOMATO: Color = Color::rgb(255, 99, 71);
    /// Transparent color { R:0, G:0, B:0, A:0 }
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
    /// Turquoise color { R:64, G:224, B:208, A:255 }
    pub const TURQUOISE: Color = Color::rgb(64, 224, 208);
    /// Violet color { R:238, G:130, B:238, A:255 }
    pub const VIOLET: Color = Color::rgb(238, 130, 238);
    /// Wheat color { R:245, G:222, B:179, A:255 }
    pub const WHEAT: Color = Color::rgb(245, 222, 179);
    /// White color { R:255, G:255, B:255, A:255 }
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    /// White Smoke color { R:245, G:245, B:245, A:255 }
    pub const WHITE_SMOKE: Color = Color::rgb(245, 245, 245);
    /// Yellow color { R:255, G:255, B:0, A:255 }
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    /// Yellow Green color { R:154, G:205, B:50, A:255 }
    pub const YELLOW_GREEN: Color = Color::rgb(154, 205, 50);
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgbf(r: f32, g: f32, b: f32) -> Self {
        Self::rgbaf(r, g, b, 1.0)
    }

    pub fn rgbaf(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        }
    }

    #[inline]
    pub const fn hex(val: u32) -> Self {
        Self {
            r: (val >> 24 & 0xFF) as u8,
            g: (val >> 16 & 0xFF) as u8,
            b: (val >> 8 & 0xFF) as u8,
            a: (val & 0xFF) as u8,
        }
    }

    // pub fn lerp(&self, other: &Self, amt: f32) -> Self {
    //     let amt = amt.clamp(0.0, 1.0);
    //     Self {
    //         r: lerp(self.r, amt, other.r),
    //         g: lerp(self.g, amt, other.g),
    //         b: lerp(self.b, amt, other.b),
    //         a: lerp(self.a, amt, other.a),
    //     }
    // }

    pub const fn to_hex(&self) -> u32 {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        let a = self.a as u32;

        (r << 24) | (g << 16) | (b << 8) | a
    }

    pub fn fade(&self, alpha: f32) -> Color {
        let alpha = alpha.clamp(0.0, 1.0);

        Self {
            a: (255.0 * alpha) as u8,
            ..*self
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: (self.r as f32 * rhs) as u8,
            g: (self.g as f32 * rhs) as u8,
            b: (self.b as f32 * rhs) as u8,
            a: (self.a as f32 * rhs) as u8,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self::rgbaf(r, g, b, a)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for [u8; 4] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}
