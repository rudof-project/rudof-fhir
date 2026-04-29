//! Load the FHIR R5 ShEx schema with rudof_lib and report what happened.
//!
//!   cargo run --release (uses schemas/fhir.shex)
//!   cargo run --release -- path/to/fhir.shex
use std::str::FromStr;
use std::time::Instant;

use anyhow::{Context, Result};
use rudof_lib::{
    formats::{InputSpec, ShExFormat},
    Rudof, RudofConfig,
};

fn main() -> Result<()> {
    let schema_input = InputSpec::from_str("schemas/fhir.shex")
        .context("Could not build an InputSpec from the schema path")?;

    let mut rudof = Rudof::new(RudofConfig::default());
    println!("rudof version: {}", rudof.version().execute());

    let started = Instant::now();
    rudof
        .load_shex_schema(&schema_input)
        .execute()
        .context("rudof could not load the ShEx schema")?;
    println!("Schema loaded into Rudof state in {:.2?}", started.elapsed());

    // rudof
    //     .serialize_shex_schema(&mut std::io::sink())
    //     .with_show_statistics(true)
    //     .with_show_dependencies(true)
    //     .with_show_time(true)
    //     .with_result_shex_format(&ShExFormat::ShExJ)
    //     .execute()
    //     .context("could not serialize the loaded ShEx schema")?;

    println!("\nDone.");
    Ok(())
}