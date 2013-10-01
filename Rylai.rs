#[crate_type = "bin"];
extern mod extra;

/* Base modules */
pub mod Crystal;
pub mod Maiden;

/* Json config is the only one yet I found in rust */
pub mod Config;

/* Supporting synchronization modes */
pub mod Git;
pub mod Gentoo_x86;

/* Main */
pub mod Main;
