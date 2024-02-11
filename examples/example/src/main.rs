use std::{fs::File, str::FromStr};

use catppuccin::{Color, ColorIdent, Flavor, FlavorIdent, Hsl, Palette, Rgb, PALETTE};
use strum::IntoEnumIterator;

const MAIN_FLAVOR: Flavor = PALETTE.mocha();

const BG: Color = MAIN_FLAVOR.base();
const FG: Color = MAIN_FLAVOR.text();

fn main() {
    // loop through each color and flavor
    for flavor_ident in FlavorIdent::iter() {
        let flavor = PALETTE.flavor(flavor_ident);

        for color_ident in ColorIdent::iter() {
            let color = flavor.color(color_ident);
            println!("{:?}", color);
        }
    }

    println!("---");

    // string to flavor and color
    let flavor_ident = FlavorIdent::from_str("frappe").unwrap();
    let color_ident = ColorIdent::from_str("sky").unwrap();
    let color = PALETTE.flavor(flavor_ident).color(color_ident);

    println!("{:?}", color);

    // import from json
    let mut palette: Palette =
        serde_json::from_reader(File::open("palette.json").unwrap()).unwrap();

    // change color to blood red
    *palette.latte_mut().red_mut() = Color::new(
        0xff0000,
        Rgb::new(0xff, 0x00, 0x00),
        Hsl::new(0.0, 1.0, 0.5),
        false,
    );
    // "red": {
    //   "hex": "#ff0000",
    //   "rgb": {
    //     "r": 255,
    //     "g": 0,
    //     "b": 0
    //   },
    //   "hsl": {
    //     "h": 0.0,
    //     "s": 1.0,
    //     "l": 0.5
    //   },
    //   "accent": false
    // },

    // export to json
    serde_json::to_writer_pretty(File::create("custom.json").unwrap(), &palette).unwrap();
}
