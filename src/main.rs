use anyhow::{Context, Result};
use planilla_bac as pbac;

fn main() -> Result<()> {
    pbac::print_version();

    let date = pbac::get_date();
    let envio = pbac::get_envio_correlative();
    let r = pbac::gen_files(date, envio);
    match r {
        Ok(()) => pbac::display_success().context(""),
        Err(e) => pbac::display_error(e.to_string()).context("Cerrando aplicación"),
    }
}