use planilla_BAC as pbac;

fn main() {
    let envio = pbac::get_envio_correlative();
    let month = pbac::get_month();
    pbac::gen_files(month, envio);

}