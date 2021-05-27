use chrono::prelude::*;
use chrono::{Datelike, Local};

use config::ConfigError;
use read_input::prelude::*;

mod config;
mod employee;
mod formatprn;
mod payment;
mod writepay;

use toml;
use std::fs::write;

fn write_config_toml(config: config::Config) {
    let t = toml::to_string_pretty(&config.excel).unwrap();
    write("config.toml", t).unwrap();
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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

pub fn get_date() -> String {
    let mut date = Local::today();
    loop {
        println!(
            "La fecha a usar es: {}-{:0>2}-{:0>2}",
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
        "La fecha a usar es {}-{:0>2}-{:0>2}",
        date.year(),
        date.month(),
        date.day()
    );

    format!("{}{:0>2}{:0>2}", date.year(), date.month(), date.day())
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

pub fn gen_files(date: String, envio: u32) -> Result<(), ConfigError> {
    let config = config::Config::new(date, envio)?;
    let employees = employee::get_employees(&config)?;
    let mut pay = payment::Payment::new(&config, &employees);
    writepay::write_outputs(&config, &employees, &mut pay)?;
    write_config_toml(config);
    Ok(())
}

pub fn display_error(error: String) -> Result<(), ConfigError> {
    println!("");
    println!("[ERROR] El programa no fue ejecutado correctamente. \r\n{}", error);
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
