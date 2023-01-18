//! Main entry point for Assimilate

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use assimilate::application::APP;

/// Boot Assimilate
fn main() {
    abscissa_core::boot(&APP);
}
