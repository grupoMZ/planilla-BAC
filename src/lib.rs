use chrono::{Datelike, Local};
use text_io::try_read;

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
pub struct Month {
    value: usize,
    string: String,
}

impl Month {
    pub fn new(value: usize) -> Month {
        let string = MONTHS[value].to_string();
        Month { value, string }
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

pub fn get_envio_correlative() -> u32 {
    loop {
        println!("");
        println!("Introduzca el 'Número de envío' actual,");
        println!("según la aplicación de planilla en línea BAC:");
        let envio: Result<u32, _> = try_read!();
        match envio {
            Ok(n) => {
                if (n < 1) || (n > 99999) {
                    println!("ERROR: número de envío inválido: {}", n);
                    println!("Introduzca un número entre 1 y 99999");
                } else {
                    return n;
                }
            },
            Err(err) => panic!("ERROR: entrada de texto invalida -- {}", err),
        }
    }
}

pub fn get_month() -> u32 {
    let now = Local::now();
    println!("Hoy es {:?}", now.to_rfc2822());
    let mut month = Month::new(now.month() as usize);
    loop {
        println!("");
        month.print_current();
        println!("Introduzca el número del mes deseado (1=Enero, 12=Diciembre, etc.)");
        let num: Result<usize, _> = try_read!();
        match num {
            Ok(n) => {
                if (n < 1) || (n > 12) {
                    println!("ERROR: número de mes inválido: {}", n);
                    println!("Introduzca un número entre 1 y 12");
                } else {
                    month.update(n);
                    month.print_confirm();
                    break;
                }
            }
            Err(err) => panic!("ERROR: entrada de texto invalida -- {}", err),
        }
    }

    month.value as u32
}

pub fn gen_files(month: u32, envio: u32) {
    let config = config::Config::new(month, envio);
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
