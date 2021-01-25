#[cfg(test)]
#[macro_use]
extern crate float_cmp;

#[cfg(test)]
mod api;

#[cfg(test)]
mod bugs;

#[cfg(test)]
mod cmdline;

#[cfg(test)]
mod errors;

#[cfg(test)]
mod intrinsic_dimensions;

#[cfg(test)]
mod loading_crash;

#[cfg(test)]
mod predicates;

#[cfg(test)]
mod primitives;

#[cfg(test)]
mod reference;

#[cfg(test)]
mod reference_utils;

#[cfg(test)]
mod render_crash;

#[cfg(test)]
mod utils;

fn main() {
    println!("Use 'cargo test' to run the tests.");
}
