#[feature(managed_boxes)];
#[crate_type = "bin"];
extern mod extra;

/* Base modules */
pub mod Moon;
pub mod Shell;
pub mod Butterfly;

/* Json config is the only one yet I found in rust */
pub mod Config;

/* Supporting synchronization modes */
pub mod Git;
pub mod Hg;
pub mod Cvs;
pub mod Gentoo_x86;

/* Main */
pub mod Main;
