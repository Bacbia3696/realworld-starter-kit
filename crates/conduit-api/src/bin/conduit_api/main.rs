//! Main entry point for ConduitApi

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use conduit_api::application::APP;

/// Boot ConduitApi
fn main() {
    abscissa_core::boot(&APP);
}
