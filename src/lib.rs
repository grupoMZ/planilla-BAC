use chrono::prelude::*;
use chrono::{Datelike, Local};

use read_input::prelude::*;

mod config;
mod employee;
mod formatprn;
mod payment;
mod writepay;

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
            .msg("Desea usar esa fecha [Y/n]")
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
                .inside_err(2000..=2100, "ERROR: Introduzca un valor entre 2000 y 2100")
                .get();
            date = date.with_year(year).expect("Año inválido");

            println!("El mes actual es {}", date.month());
            let month = input()
                .repeat_msg(
                    "Presione Intro para usar el mes actual o introduzca un mes diferente: ",
                )
                .default(date.month())
                .inside_err(1..=12, "ERROR: Introduzca un valor entre 1 y 12")
                .get();
            date = date.with_month(month).expect("Mes inválido");

            println!("El día de hoy es {}", date.day());
            let day = input()
                .repeat_msg(
                    "Presione Intro para usar el día de hoy o introduzca un día diferente: ",
                )
                .default(date.day())
                .inside_err(1..=31, "ERROR: Introduzca un valor entre 1 y 31")
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
    println!("Introduzca el 'Número de envío' actual,");
    println!("según la aplicación de planilla en línea BAC:");
    let envio: u32 = input().get();
    envio
}

pub fn get_month() -> u32 {
    let now = Local::today();
    println!("Hoy es {:?}", now);
    let mut month = Date::new(now.month() as usize);
    println!("");
    month.print_current();
    let num = input()
        .msg("Introduzca el número del mes deseado (1=Enero, 12=Diciembre, etc.)")
        .default(month.value)
        .get();

    month.value as u32
}

pub fn gen_files(date: String, envio: u32) {
    let config = config::Config::new(date, envio);
    let employees = employee::get_employees(&config).expect("Error leyendo empleados");
    let mut pay = payment::Payment::new(&config, &employees);
    writepay::write_salario(&config, &employees, &mut pay)
        .expect("No pude escribir el achivo pago de propinas");
    let mut pay = payment::Payment::new(&config, &employees);
    writepay::write_viatico(&config, &employees, &mut pay)
        .expect("No pude escribir el achivo pago de propinas");
    let mut pay = payment::Payment::new(&config, &employees);
    writepay::write_propina(&config, &employees, &mut pay)
        .expect("No pude escribir el achivo pago de propinas");
}

pub fn wait_for_user() {
    println!("");
    println!("Programa ejecutado correctamente");
    let _: String = input().msg("Presione Enter para cerrar esta ventana").get();
}
