use std::{
    env,
    fs::{self, File},
    path::Path,
};

use case::CaseExt;

const PALETTE_VERSION: &str = "v1.0.3";

type Palette = indexmap::IndexMap<String, Flavor>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Flavor {
    name: String,
    dark: bool,
    colors: indexmap::IndexMap<String, Color>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Color {
    hex: String,
    rgb: Rgb,
    hsl: Hsl,
    accent: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Hsl {
    h: f32,
    s: f32,
    l: f32,
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let palette_path = Path::new(&out_dir).join(format!("{PALETTE_VERSION}-palette.json"));

    let palette: Palette = if let Ok(file) = File::open(&palette_path) {
        serde_json::from_reader(file).unwrap()
    } else {
        let url = format!(
            "https://raw.githubusercontent.com/catppuccin/palette/{PALETTE_VERSION}/palette.json"
        );
        let palette: Palette = ureq::get(&url).call().unwrap().into_json().unwrap();
        serde_json::to_writer(File::create(&palette_path).unwrap(), &palette).unwrap();
        palette
    };

    let tokens = vec![
        palette_struct(&palette),
        colors_struct(&palette),
        flavor_name_enum(&palette),
        flavor_ident_enum(&palette),
        color_ident_enum(&palette),
        palette_impl(&palette),
        flavor_impl(&palette),
        palette_const(&palette),
    ];

    let syntax_tree = syn::parse2(tokens.into_iter().collect()).unwrap();
    let output = prettyplease::unparse(&syntax_tree);

    let dest_path = Path::new(&out_dir).join("palette.rs");
    fs::write(dest_path, output).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

fn palette_struct(palette: &Palette) -> proc_macro2::TokenStream {
    let flavors = palette.keys().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.to_snake());

        quote::quote! {
            #flavor_ident: Flavor
        }
    });

    quote::quote! {
        #[derive(Debug, Clone, Copy)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Palette {
            #(#flavors),*
        }
    }
}

fn colors_struct(palette: &Palette) -> proc_macro2::TokenStream {
    let colors = palette
        .values()
        .next()
        .unwrap()
        .colors
        .keys()
        .map(|color_name| {
            let color_ident = quote::format_ident!("{}", color_name.to_snake());

            quote::quote! {
                #[cfg_attr(feature = "serde", serde(rename = #color_name))]
                #color_ident: Color
            }
        });

    quote::quote! {
        #[derive(Debug, Clone, Copy)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Colors {
            #(#colors),*
        }
    }
}

fn flavor_name_enum(palette: &Palette) -> proc_macro2::TokenStream {
    let flavors = palette.values().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.name.to_camel());
        let flavor_name = &flavor.name;

        quote::quote! {
            #[cfg_attr(feature = "serde", serde(rename = #flavor_name))]
            #[cfg_attr(feature = "strum", strum(serialize = #flavor_name))]
            #flavor_ident
        }
    });

    quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "strum", derive(strum_macros::EnumString, strum_macros::Display, strum_macros::FromRepr, strum_macros::AsRefStr, strum_macros::EnumIter, strum_macros::EnumCount, strum_macros::VariantArray, strum_macros::VariantNames, strum_macros::EnumTable))]
        pub enum FlavorName {
            #(#flavors),*
        }
    }
}

fn flavor_ident_enum(palette: &Palette) -> proc_macro2::TokenStream {
    let flavors = palette.keys().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.to_camel());

        quote::quote! {
            #[cfg_attr(feature = "serde", serde(rename = #flavor))]
            #[cfg_attr(feature = "strum", strum(serialize = #flavor))]
            #flavor_ident
        }
    });

    quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "strum", derive(strum_macros::EnumString, strum_macros::Display, strum_macros::FromRepr, strum_macros::AsRefStr, strum_macros::EnumIter, strum_macros::EnumCount, strum_macros::VariantArray, strum_macros::VariantNames, strum_macros::EnumTable))]
        pub enum FlavorIdent {
            #(#flavors),*
        }
    }
}

fn color_ident_enum(palette: &Palette) -> proc_macro2::TokenStream {
    let colors = palette
        .values()
        .next()
        .unwrap()
        .colors
        .keys()
        .map(|color_name| {
            let color_ident = quote::format_ident!("{}", color_name.to_camel());

            quote::quote! {
                #[cfg_attr(feature = "serde", serde(rename = #color_name))]
                #[cfg_attr(feature = "strum", strum(serialize = #color_name))]
                #color_ident
            }
        });

    quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "strum", derive(strum_macros::EnumString, strum_macros::Display, strum_macros::FromRepr, strum_macros::AsRefStr, strum_macros::EnumIter, strum_macros::EnumCount, strum_macros::VariantArray, strum_macros::VariantNames, strum_macros::EnumTable))]
        pub enum ColorIdent {
            #(#colors),*
        }
    }
}

fn palette_impl(palette: &Palette) -> proc_macro2::TokenStream {
    let flavors = palette.keys().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.to_snake());

        quote::quote! {
            #flavor_ident
        }
    });

    let flavor_params = palette.keys().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.to_snake());

        quote::quote! {
            #flavor_ident: Flavor
        }
    });

    let flavor_fns = palette.keys().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.to_snake());
        let flavor_fn = quote::format_ident!("{}", flavor.to_snake());

        quote::quote! {
            pub const fn #flavor_fn(&self) -> Flavor {
                self.#flavor_ident
            }
        }
    });

    let flavor_fns_mut = palette.keys().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.to_snake());
        let flavor_fn = quote::format_ident!("{}_mut", flavor.to_snake());

        quote::quote! {
            pub fn #flavor_fn(&mut self) -> &mut Flavor {
                &mut self.#flavor_ident
            }
        }
    });

    let flavor_matches = palette.keys().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.to_camel());
        let flavor_fn = quote::format_ident!("{}", flavor.to_snake());

        quote::quote! {
            FlavorIdent::#flavor_ident => self.#flavor_fn()
        }
    });

    let flavor_matches_mut = palette.keys().map(|flavor| {
        let flavor_ident = quote::format_ident!("{}", flavor.to_camel());
        let flavor_fn = quote::format_ident!("{}_mut", flavor.to_snake());

        quote::quote! {
            FlavorIdent::#flavor_ident => self.#flavor_fn()
        }
    });

    quote::quote! {
        impl Palette {
            pub const fn new(#(#flavor_params),*) -> Self {
                Self {
                    #(#flavors),*
                }
            }

            #(#flavor_fns)*

            #(#flavor_fns_mut)*

            pub const fn flavor(&self, flavor: FlavorIdent) -> Flavor {
                match flavor {
                    #(#flavor_matches),*
                }
            }

            pub fn flavor_mut(&mut self, flavor: FlavorIdent) -> &mut Flavor {
                match flavor {
                    #(#flavor_matches_mut),*
                }
            }
        }
    }
}

fn flavor_impl(palette: &Palette) -> proc_macro2::TokenStream {
    let color_idents: Vec<_> = palette.values().next().unwrap().colors.keys().collect();

    let colors = color_idents.iter().map(|color| {
        let color_ident = quote::format_ident!("{}", color.to_snake());

        quote::quote! {
            #color_ident
        }
    });

    let color_params = color_idents.iter().map(|color| {
        let color_ident = quote::format_ident!("{}", color.to_snake());

        quote::quote! {
            #color_ident: Color
        }
    });

    let color_fns = color_idents.iter().map(|color| {
        let color_ident = quote::format_ident!("{}", color.to_snake());
        let color_fn = quote::format_ident!("{}", color.to_snake());

        quote::quote! {
            pub const fn #color_fn(&self) -> Color {
                self.colors.#color_ident
            }
        }
    });

    let color_fns_mut = color_idents.iter().map(|color| {
        let color_ident = quote::format_ident!("{}", color.to_snake());
        let color_fn = quote::format_ident!("{}_mut", color.to_snake());

        quote::quote! {
            pub fn #color_fn(&mut self) -> &mut Color {
                &mut self.colors.#color_ident
            }
        }
    });

    let color_matches = color_idents.iter().map(|color| {
        let color_ident = quote::format_ident!("{}", color.to_camel());
        let color_fn = quote::format_ident!("{}", color.to_snake());

        quote::quote! {
            ColorIdent::#color_ident => self.#color_fn()
        }
    });

    let color_matches_mut = color_idents.iter().map(|color| {
        let color_ident = quote::format_ident!("{}", color.to_camel());
        let color_fn = quote::format_ident!("{}_mut", color.to_snake());

        quote::quote! {
            ColorIdent::#color_ident => self.#color_fn()
        }
    });

    quote::quote! {
        impl Flavor {
            pub const fn new(name: FlavorName, dark: bool, #(#color_params),*) -> Self {
                let colors = Colors {
                    #(#colors),*
                };

                Self {
                    name,
                    dark,
                    colors
                }
            }

            #(#color_fns)*

            #(#color_fns_mut)*

            pub const fn color(&self, color: ColorIdent) -> Color {
                match color {
                    #(#color_matches),*
                }
            }

            pub fn color_mut(&mut self, color: ColorIdent) -> &mut Color {
                match color {
                    #(#color_matches_mut),*
                }
            }
        }
    }
}

fn palette_const(palette: &Palette) -> proc_macro2::TokenStream {
    let flavor_params = palette.values().map(|flavor| {
        let color_params = flavor.colors.values().map(|color| {
            println!("{}", color.hex);
            let hex = u32::from_str_radix(color.hex.strip_prefix('#').unwrap(), 16).unwrap();
            let r = color.rgb.r;
            let g = color.rgb.g;
            let b = color.rgb.b;
            let h = color.hsl.h;
            let s = color.hsl.s;
            let l = color.hsl.l;
            let accent = color.accent;

            quote::quote! {
                Color::new(
                    #hex,
                    Rgb::new(#r, #g, #b),
                    Hsl::new(#h, #s, #l),
                    #accent,
                )
            }
        });

        let flavor_name = quote::format_ident!("{}", flavor.name.to_camel());
        let dark = flavor.dark;

        quote::quote! {
            Flavor::new(
                FlavorName::#flavor_name,
                #dark,
                #(#color_params),*
            )
        }
    });

    quote::quote! {
        pub const PALETTE: Palette = Palette::new(#(#flavor_params),*);
    }
}
