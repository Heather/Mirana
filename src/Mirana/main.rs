#[link(name = "Mirana",
       vers = "0.1.5",
       author = "Heather Cynede",
       url = "https://github.com/Heather/Mirana")];

#[comment = "Mirana VCS sync"];
#[license = "LGPL2"];

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

pub mod Shades;
pub mod VcsImpl;

pub mod StarStorm;
pub mod Core;

/* Main */
pub mod Mirana;
