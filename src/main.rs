use anyhow::{Context, Result};
use planilla_bac as pbac;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("Ejecutando archivo: \"{}\"", args[0]);

    pbac::print_version();

    let date = pbac::get_date();
    let envio = pbac::get_envio_correlative();
    let config = pbac::get_config_file_name(args);
    let r = pbac::gen_files(date, envio, config);
    match r {
        Ok(()) => pbac::display_success().context(""),
        Err(e) => pbac::display_error(e.to_string()).context("Cerrando aplicación"),
    }
}
