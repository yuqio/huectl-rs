use crate::{arg::value, output::Light as OutputLight, output::Scan as OutputScan, util};
use huelib::resource::{light, Modifier};
use huelib::Color;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Arg {
    /// Modifies the state and attributes of a light
    Set(Set),
    /// Prints the state and attributes of a light
    Get(Get),
    /// Searches for new lights
    Search(Search),
    /// Deletes a light
    Delete(Delete),
}

#[derive(Debug, StructOpt)]
pub struct Set {
    /// Identifier of the light
    pub id: String,
    /// Turns the light on
    #[structopt(long)]
    on: bool,
    /// Turns the light off
    #[structopt(long)]
    off: bool,
    /// Sets the brightness of the light in percentage
    #[structopt(long, short, allow_hyphen_values = true)]
    brightness: Option<value::Brightness>,
    /// Sets the hue of the light
    #[structopt(long, allow_hyphen_values = true)]
    hue: Option<value::Hue>,
    /// Sets the saturation of the light in percentage
    #[structopt(long, short, allow_hyphen_values = true)]
    saturation: Option<value::Saturation>,
    /// Sets the color temperature of the light
    #[structopt(long, short = "t", allow_hyphen_values = true)]
    color_temperature: Option<value::ColorTemperature>,
    /// Sets the x and y coordinates in the color space of the light
    #[structopt(long, short, name = "coordinate", min_values = 2, max_values = 2)]
    color_space_coordinates: Option<Vec<f32>>,
    /// Sets the color of the light with red, green, and blue values
    #[structopt(long, short = "r", min_values = 3, max_values = 3)]
    color_rgb: Option<Vec<u8>>,
    /// Sets the color of the light with a hex value
    #[structopt(long, short = "x")]
    color_hex: Option<value::ColorHex>,
    /// Sets the alert effect of the light
    #[structopt(long, short, case_insensitive = true, possible_values = value::Alert::variants())]
    alert: Option<value::Alert>,
    /// Sets the dynamic effect of the light
    #[structopt(long, short, case_insensitive = true, possible_values = value::Effect::variants())]
    effect: Option<value::Effect>,
    /// Sets the transition time of the light
    #[structopt(long)]
    transition_time: Option<u16>,
    /// Renames the light
    #[structopt(long, short)]
    name: Option<String>,
}

impl Set {
    pub fn to_state_modifier(&self) -> light::StateModifier {
        let mut modifier = light::StateModifier::new();
        if self.on {
            modifier = modifier.on(true);
        } else if self.off {
            modifier = modifier.on(false);
        }
        if let Some(v) = &self.brightness {
            modifier = modifier.brightness(v.0, v.1);
        }
        if let Some(v) = &self.hue {
            modifier = modifier.hue(v.0, v.1);
        }
        if let Some(v) = &self.saturation {
            modifier = modifier.saturation(v.0, v.1);
        }
        if let Some(v) = &self.color_space_coordinates {
            modifier = modifier.color(Color::from_space_coordinates(v[0], v[1]));
        }
        if let Some(v) = &self.color_rgb {
            modifier = modifier.color(Color::from_rgb(v[0], v[1], v[2]));
        }
        if let Some(v) = &self.color_hex {
            modifier = modifier.color(v.0);
        }
        if let Some(v) = &self.color_temperature {
            modifier = modifier.color_temperature(v.0, v.1);
        }
        if let Some(v) = &self.alert {
            modifier = modifier.alert(v.0);
        }
        if let Some(v) = &self.effect {
            modifier = modifier.effect(v.0);
        }
        if let Some(v) = self.transition_time {
            modifier = modifier.transition_time(v);
        }
        modifier
    }

    pub fn to_attribute_modifier(&self) -> light::AttributeModifier {
        let mut modifier = light::AttributeModifier::new();
        if let Some(v) = &self.name {
            modifier = modifier.name(v);
        }
        modifier
    }
}

pub fn set(arg: Set) {
    let bridge = util::get_bridge();
    let mut responses = Vec::new();
    let state_modifier = arg.to_state_modifier();
    if !state_modifier.is_empty() {
        responses.extend(match bridge.set_light_state(&arg.id, &state_modifier) {
            Ok(v) => v,
            Err(e) => exit!("Error occured while modifying the state of the light", e),
        });
    }
    let attribute_modifier = arg.to_attribute_modifier();
    if !attribute_modifier.is_empty() {
        responses.extend(
            match bridge.set_light_attribute(&arg.id, &attribute_modifier) {
                Ok(v) => v,
                Err(e) => exit!("Error occured while modifying attributes of the light", e),
            },
        );
    }
    for i in responses {
        println!("{}", i);
    }
}

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Identifier of the light, if omitted all lights are selected
    pub id: Option<String>,
}

pub fn get(arg: Get) {
    let bridge = util::get_bridge();
    match arg.id {
        Some(v) => match bridge.get_light(&v) {
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputLight::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get light", e),
        },
        None => match bridge.get_all_lights() {
            Ok(v) => {
                let lights: Vec<OutputLight> = v.into_iter().map(OutputLight::from).collect();
                println!("{}", serde_json::to_string_pretty(&lights).unwrap());
            }
            Err(e) => exit!("Failed to get lights", e),
        },
    };
}

#[derive(Debug, StructOpt)]
pub struct Search {
    /// Prints the lights that were discovered by the last search
    #[structopt(long, short)]
    pub get: bool,
}

pub fn search(arg: Search) {
    let bridge = util::get_bridge();
    if arg.get {
        match bridge.get_new_lights() {
            Ok(v) => println!(
                "{}",
                serde_json::to_string_pretty(&OutputScan::from(v)).unwrap()
            ),
            Err(e) => exit!("Failed to get new lights", e),
        };
    } else {
        match bridge.search_new_lights(None) {
            Ok(_) => println!("Searching for new lights..."),
            Err(e) => exit!("Failed to search for new lights", e),
        };
    }
}

#[derive(Debug, StructOpt)]
pub struct Delete {
    /// Identifier of the light
    pub id: String,
}

pub fn delete(arg: Delete) {
    match util::get_bridge().delete_light(&arg.id) {
        Ok(_) => println!("Deleted light {}", arg.id),
        Err(e) => exit!("Failed to delete light", e),
    };
}
