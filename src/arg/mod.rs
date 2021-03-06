mod config;
mod group;
mod light;
mod resourcelink;
mod rule;
mod scene;
mod schedule;
mod sensor;
mod value;

use std::{fmt, net::IpAddr};
use structopt::StructOpt;

pub fn exec() {
    let args = Args::from_args();
    match args.subcommand {
        Subcommand::Discover => discover(),
        Subcommand::Register(v) => register(v),
        Subcommand::Config(v) => match v {
            config::Arg::Set(v) => config::set(v),
            config::Arg::Get => config::get(),
        },
        Subcommand::Light(v) => match v {
            light::Arg::Set(v) => light::set(v),
            light::Arg::Get(v) => light::get(v),
            light::Arg::Search(v) => light::search(v),
            light::Arg::Delete(v) => light::delete(v),
        },
        Subcommand::Group(v) => match v {
            group::Arg::Set(v) => group::set(v),
            group::Arg::Get(v) => group::get(v),
            group::Arg::Create(v) => group::create(v),
            group::Arg::Delete(v) => group::delete(v),
        },
        Subcommand::Resourcelink(v) => match v {
            resourcelink::Arg::Set(v) => resourcelink::set(v),
            resourcelink::Arg::Get(v) => resourcelink::get(v),
            resourcelink::Arg::Create(v) => resourcelink::create(v),
            resourcelink::Arg::Delete(v) => resourcelink::delete(v),
        },
        Subcommand::Rule(v) => match v {
            rule::Arg::Set(v) => rule::set(v),
            rule::Arg::Get(v) => rule::get(v),
            rule::Arg::Create(v) => rule::create(v),
            rule::Arg::Delete(v) => rule::delete(v),
        },
        Subcommand::Scene(v) => match v {
            scene::Arg::Set(v) => scene::set(v),
            scene::Arg::Get(v) => scene::get(v),
            scene::Arg::Create(v) => scene::create(v),
            scene::Arg::Delete(v) => scene::delete(v),
        },
        Subcommand::Schedule(v) => match v {
            schedule::Arg::Set(v) => schedule::set(v),
            schedule::Arg::Get(v) => schedule::get(v),
            schedule::Arg::Create(v) => schedule::create(v),
            schedule::Arg::Delete(v) => schedule::delete(v),
        },
        Subcommand::Sensor(v) => match v {
            sensor::Arg::Set(v) => sensor::set(v),
            sensor::Arg::Get(v) => sensor::get(v),
            sensor::Arg::Search(v) => sensor::search(v),
            sensor::Arg::Delete(v) => sensor::delete(v),
        },
    };
}

/// A command line interface to Philips Hue
#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Discovers bridges in the local network
    Discover,
    /// Registers a new user on a bridge
    Register(Register),
    /// Modifies or prints the bridge configuration
    Config(config::Arg),
    /// Modifies, prints, searches or deletes lights
    Light(light::Arg),
    /// Modifies, prints, creates or deletes groups
    Group(group::Arg),
    /// Modifier, prints, creates or deletes resourcelinks
    Resourcelink(resourcelink::Arg),
    /// Modifier, prints, creates or deletes rules
    Rule(rule::Arg),
    /// Modifies, prints, creates or deletes scenes
    Scene(scene::Arg),
    /// Modifies, prints, creates or deletes schedules
    Schedule(schedule::Arg),
    /// Modifies, prints, searches or deletes sensors
    Sensor(sensor::Arg),
}

pub fn discover() {
    let ip_addresses = match huelib::bridge::discover() {
        Ok(v) => v,
        Err(e) => exit!("Failed to discover bridges", e),
    };
    for i in ip_addresses {
        println!("{}", i);
    }
}

#[derive(Debug, StructOpt)]
pub struct Register {
    /// IP address of the bridge, if omitted the user will be registered on the first discovered
    /// bridge
    pub ip_address: Option<IpAddr>,
    /// Sets environment variables for the current session
    #[structopt(long, short)]
    pub set_env: bool,
}

pub fn register(arg: Register) {
    let ip_address = match arg.ip_address {
        Some(v) => v,
        None => match huelib::bridge::discover() {
            Ok(mut v) => match v.pop() {
                Some(v) => v,
                None => exit!("No bridges were found"),
            },
            Err(e) => exit!("Failed to discover bridges", e),
        },
    };
    let user = match huelib::bridge::register_user(ip_address, "huectl-rs", false) {
        Ok(v) => v,
        Err(e) => exit!(
            format!(
                "Failed to register user on bridge with the IP address '{}'",
                ip_address
            ),
            e
        ),
    };
    if arg.set_env {
        std::env::set_var(crate::config::VAR_BRIDGE_IP, ip_address.to_string());
        std::env::set_var(crate::config::VAR_BRIDGE_USERNAME, user.name);
    } else {
        println!("{}={}", crate::config::VAR_BRIDGE_IP, ip_address);
        println!("{}={}", crate::config::VAR_BRIDGE_USERNAME, user.name);
    }
}

#[derive(Clone, Debug)]
pub struct ParseError {
    description: String,
}

impl ParseError {
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_owned(),
        }
    }

    pub fn from_integer_value<T: fmt::Display>(max_value: &T) -> Self {
        Self::new(&format!(
            "The value must be an integer between 0 and {} and can have '-' or '+' as prefix.",
            max_value
        ))
    }
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description)
    }
}
