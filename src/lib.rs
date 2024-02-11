include!(concat!(env!("OUT_DIR"), "/palette.rs"));

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Flavor {
    name: FlavorName,
    dark: bool,
    colors: Colors,
}
impl Flavor {
    pub const fn name(&self) -> FlavorName {
        self.name
    }
    pub const fn dark(&self) -> bool {
        self.dark
    }
}
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    #[cfg_attr(feature = "serde", serde(with = "hex"))]
    hex: u32,
    rgb: Rgb,
    hsl: Hsl,
    accent: bool,
}
impl Color {
    pub const fn new(hex: u32, rgb: Rgb, hsl: Hsl, accent: bool) -> Self {
        Self {
            hex,
            rgb,
            hsl,
            accent,
        }
    }
    pub const fn hex(&self) -> u32 {
        self.hex
    }
    pub const fn rgb(&self) -> Rgb {
        self.rgb
    }
    pub const fn hsl(&self) -> Hsl {
        self.hsl
    }
    pub const fn accent(&self) -> bool {
        self.accent
    }
    pub fn hex_mut(&mut self) -> &mut u32 {
        &mut self.hex
    }
    pub fn rgb_mut(&mut self) -> &mut Rgb {
        &mut self.rgb
    }
    pub fn hsl_mut(&mut self) -> &mut Hsl {
        &mut self.hsl
    }
    pub fn hex_string(&self) -> String {
        format!("#{:06x}", self.hex)
    }
}
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}
impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub const fn r(&self) -> u8 {
        self.r
    }
    pub const fn g(&self) -> u8 {
        self.g
    }
    pub const fn b(&self) -> u8 {
        self.b
    }
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.r
    }
    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.g
    }
    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.b
    }
}
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hsl {
    h: f32,
    s: f32,
    l: f32,
}
impl Hsl {
    pub const fn new(h: f32, s: f32, l: f32) -> Self {
        Self { h, s, l }
    }
    pub const fn h(&self) -> f32 {
        self.h
    }
    pub const fn s(&self) -> f32 {
        self.s
    }
    pub const fn l(&self) -> f32 {
        self.l
    }
    pub fn h_mut(&mut self) -> &mut f32 {
        &mut self.h
    }
    pub fn s_mut(&mut self) -> &mut f32 {
        &mut self.s
    }
    pub fn l_mut(&mut self) -> &mut f32 {
        &mut self.l
    }
}
#[cfg(feature = "serde")]
mod hex {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;

        Ok(u32::from_str_radix(s.strip_prefix('#').unwrap(), 16).unwrap())
    }
    pub fn serialize<S>(hex: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&format!("#{:06x}", hex))
    }
}
