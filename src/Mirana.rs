#[feature(managed_boxes)];
#[crate_type = "bin"];
extern mod extra;

/* Base modules */
pub mod Moon;
pub mod Shell;
pub mod Butterfly;
pub mod Misc;

/* Json config is the only one yet I found in rust */
pub mod Config;
pub mod Shade;

/* Main */
pub mod Main;
