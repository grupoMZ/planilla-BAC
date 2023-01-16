use planilla_bac as pbac;
use std::fs::read;

fn assert_files_eq(fref: &str, fgen: &str) {
    let bgen = read(fgen).unwrap();
    let bref = read(fref).unwrap();
    for (i, _) in bref.iter().enumerate() {
        if bref[i] != bgen[i] {
            println!(
                "Generated file {} does not match reference file {}.",
                fgen, fref
            );
            println!("Byte {} differ", i);
            assert_eq!(bref[i] as char, bgen[i] as char);
        }
    }
}

#[test]
fn output_prn_files() {
    let date_out = String::from("20210430");
    let month_in = String::from("04");  // ABR
    let envio = 17;
    let config = "config.json".to_string();
    pbac::gen_files(date_out,  month_in, envio, config).unwrap();
    assert_files_eq(
        "./tests/00017_test_salario_ABR.prn",
        "./tests/pago_bac/00017 pago BAC salario ABR.prn",
    );
    assert_files_eq(
        "./tests/00018_test_viatico_ABR.prn",
        "./tests/pago_bac/00018 pago BAC viatico ABR.prn",
    );
    assert_files_eq(
        "./tests/00019_test_propina_ABR.prn",
        "./tests/pago_bac/00019 pago BAC propina ABR.prn",
    );
}

#[test]
fn output_prn_files_propina() {
    // Test generating an output file in April, using input data file from March
    let date_out = String::from("20210430");
    let month_in = String::from("03");  // MAR
    let envio = 19;
    let config = "config_propina.json".to_string();
    pbac::gen_files(date_out,  month_in, envio, config).unwrap();
    assert_files_eq(
        "./tests/00019_test_propina_MAR.prn",
        "./tests/pago_bac/00019 pago BAC propina MAR.prn",
    );
}
