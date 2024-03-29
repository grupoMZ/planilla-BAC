use chrono::{Datelike, Local};

use config::ConfigError;
use read_input::prelude::*;

mod config;
mod employee;
mod formatprn;
mod payment;
mod writepay;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const CONFIG_FNAME: &'static str = "config.json";

const MONTHS: &[&str] = &[
    "",
    "ENERO",
    "FEBRERO",
    "MARZO",
    "ABRIL",
    "MAYO",
    "JUNIO",
    "JULIO",
    "AGOSTO",
    "SEPTIEMBRE",
    "OCTUBRE",
    "NOVIEMBRE",
    "DICIEMBRE",
];
pub struct Date {
    value: usize,
    string: String,
}

impl Date {
    pub fn new(value: usize) -> Date {
        let string = MONTHS[value].to_string();
        Date { value, string }
    }

    pub fn update(&mut self, value: usize) {
        self.value = value;
        self.string = MONTHS[value].to_string();
    }

    pub fn print_current(&self) {
        println!("El mes actual es {} ({}).", self.value, self.string);
    }

    pub fn print_confirm(&self) {
        println!(
            "Se generarán archivos para pago de mes {} ({}).",
            self.value, self.string
        );
        println!("");
    }
}

pub fn print_version() {
    println!("");
    println!("Programa de pago de Planilla BAC");
    println!("VERSION: {}", VERSION);
    println!("");
}

pub fn get_output_date() -> String {
    let mut date = Local::today();
    loop {
        println!(
            "La fecha de pago a usar dentro del archivo .prn a ser generado es: {}-{:0>2}-{:0>2}",
            date.year(),
            date.month(),
            date.day()
        );
        let usetoday = input()
            .msg("Desea usar esa fecha [Y/n]? ")
            .default("Y".to_string())
            .get();

        if usetoday.eq("Y") {
            break;
        } else {
            println!("El año actual es {}", date.year());
            let year = input()
                .repeat_msg(
                    "Presione Intro para usar el año actual o introduzca un año diferente: ",
                )
                .default(date.year())
                .inside_err(2000..=2100, "[ERROR] Introduzca un valor entre 2000 y 2100")
                .get();
            date = date.with_year(year).expect("Año inválido");

            println!("El mes actual es {}", date.month());
            let month = input()
                .repeat_msg(
                    "Presione Intro para usar el mes actual o introduzca un mes diferente: ",
                )
                .default(date.month())
                .inside_err(1..=12, "[ERROR] Introduzca un valor entre 1 y 12")
                .get();
            date = date.with_month(month).expect("Mes inválido");

            println!("El día de hoy es {}", date.day());
            let day = input()
                .repeat_msg(
                    "Presione Intro para usar el día de hoy o introduzca un día diferente: ",
                )
                .default(date.day())
                .inside_err(1..=31, "[ERROR] Introduzca un valor entre 1 y 31")
                .get();
            date = date.with_day(day).expect("Día inválido");
        }
    }

    println!("");
    println!(
        "La fecha de pago a usar dentro del archivo .prn a ser generado es {}-{:0>2}-{:0>2}",
        date.year(),
        date.month(),
        date.day()
    );
    println!("");

    format!("{}{:0>2}{:0>2}", date.year(), date.month(), date.day())
}

pub fn get_input_month() -> String {
    let mut date = Local::today();
    loop {
        let month = date.month();
        let month_s= MONTHS[month as usize].to_string();
        println!(
            "El mes a usar del archivo .xlsx con los datos de los empleados es: {:0>2} ({})",
            month, month_s
        );
        let usethismonth = input()
            .msg("Desea usar ese mes [Y/n]? ")
            .default("Y".to_string())
            .get();

        if usethismonth.eq("Y") {
            break;
        } else {
            println!("El mes actual es {} ({})", month, month_s);
            let new_month = input()
                .repeat_msg(
                    "Presione Intro para usar el mes actual o introduzca un mes diferente: ",
                )
                .default(month)
                .inside_err(1..=12, "[ERROR] Introduzca un valor entre 1 y 12")
                .get();
            date = date.with_month(new_month).expect("Mes inválido");
        }
    }

    let month = date.month();
    let month_s= MONTHS[month as usize].to_string();
    println!("");
    println!(
        "El mes a usar del archivo .xlsx con los datos de los empleados es: {:0>2} ({})",
        month, month_s
    );
    println!("");

    format!("{:0>2}", month)
}

pub fn get_envio_correlative() -> u32 {
    println!("");
    let envio: u32 = input()
    .repeat_msg("Introduzca el 'Número de envío' actual,\r\nsegún la aplicación de planilla en línea BAC: ")
    .inside_err(1..=99999, "[ERROR] Introduzca un valor entre 1 y 99999\r\n")
    .err("[ERROR] Introduzca un valor numérico entre 1 y 99999\r\n")
    //.default(1)  use this for tests
    .get();

    println!("");
    println!("El número de envio a usar es {}", envio);
    println!("");
    envio
}

pub fn get_config_file_name(args: Vec<String>) -> String {
    // The name of the configuration file can be optionally
    // specified as the first command line argument to the program
    let config = if args.len() == 2 {
        args[1].clone()
    } else {
        CONFIG_FNAME.to_string()
    };

    println!("");
    println!("El archivo de configuración a usar es: \"{}\"", config);
    println!("");

    config
}

pub fn gen_files(date_out: String, month_in: String, envio: u32, config: String) -> Result<(), ConfigError> {
    let config = config::Config::new(date_out, month_in, envio, config)?;
    let employees = employee::get_employees(&config)?;
    let mut pay = payment::Payment::new(&config, &employees);
    writepay::write_outputs(&config, &employees, &mut pay)?;
    Ok(())
}

pub fn display_error(error: String) -> Result<(), ConfigError> {
    println!("");
    println!(
        "[ERROR] El programa no fue ejecutado correctamente. \r\n{}",
        error
    );
    println!("");
    let _: String = input().msg("Presione Enter para cerrar esta ventana").get();
    return Err(ConfigError::EndError);
}

pub fn display_success() -> Result<(), ConfigError> {
    println!("");
    println!("[INFO] Programa ejecutado correctamente");
    let _: String = input().msg("Presione Enter para cerrar esta ventana").get();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_config_file_name_default() {
        let fname_in = CONFIG_FNAME.to_string();
        let args = vec!["program_name".to_string()];
        let fname_out = get_config_file_name(args);
        assert_eq!(&fname_in, &fname_out);
    }

    #[test]
    fn get_config_file_name_arg() {
        let fname_in = "xyz.json".to_string();
        let args = vec!["program_name".to_string(), fname_in.clone()];
        let fname_out = get_config_file_name(args);
        assert_eq!(&fname_in, &fname_out);
    }
}
