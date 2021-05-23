use planilla_bac as pbac;

fn main() {
    pbac::print_version();

    let month = pbac::get_month();
    let envio = pbac::get_envio_correlative();
    pbac::gen_files(month, envio);
}