//! Load the FHIR R5 ShEx schema with rudof_lib and report what happened.
//!
//!   cargo run --release (uses schemas/fhir.shex)
//!   cargo run --release -- path/to/fhir.shex
use std::str::FromStr;
use std::time::Instant;

use anyhow::{Context, Result};
use rudof_lib::{
    formats::InputSpec,
    Rudof, RudofConfig,
};

fn main() -> Result<()> {
    let mut rudof = Rudof::new(RudofConfig::default());
    println!("rudof version: {}", rudof.version().execute());

    let data_input = InputSpec::from_str("examples/account-example.ttl")
        .context("Could not build an InputSpec from the data path")?;

    let schema_input = InputSpec::from_str("schemas/fhir.shex")
        .context("Could not build an InputSpec from the schema path")?;

    let shapemap_input = InputSpec::from_str("examples/account-example.sm")
        .context("Could not build an InputSpec from the ShapeMap path")?;

    let started_data = Instant::now();
    rudof
        .load_data()
        .with_data(&[data_input])
        .execute()
        .context("rudof could not load the data file")?;
    println!("Data loaded into Rudof state in {:.2?}", started_data.elapsed());

    let started_schema = Instant::now();
    rudof
        .load_shex_schema(&schema_input)
        .execute()
        .context("rudof could not load the ShEx schema")?;
    println!("Schema loaded into Rudof state in {:.2?}", started_schema.elapsed());
    
    let started_shapemap = Instant::now();
    rudof
        .load_shapemap(&shapemap_input)
        .execute()
        .context("rudof could not load the ShapeMap")?;
    println!("ShapeMap loaded into Rudof state in {:.2?}", started_shapemap.elapsed());

    let started_validation = Instant::now();
    rudof.validate_shex().execute().context("rudof could not validate the data against the schema")?;
    println!("Validation completed in {:.2?}", started_validation.elapsed());

    rudof.serialize_shex_validation_results(&mut std::io::stdout()).execute().context("rudof could not report the validation results")?;

    println!("\nDone.");
    Ok(())
}