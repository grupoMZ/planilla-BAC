use std::process::exit;

use planilla_bac as pbac;

fn main() {
    pbac::print_version();

    let date = pbac::get_date();
    let envio = pbac::get_envio_correlative();
    pbac::gen_files(date, envio);
    pbac::wait_for_user();
}