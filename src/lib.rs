
use calamine::{open_workbook, DataType, Error, RangeDeserializerBuilder, Reader, Xlsx};
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use text_io::read;

mod config;
mod employee;
mod formatprn;
mod payment;
mod writepay;

pub fn get_envio_correlative() -> u32 {
    println!("Introduzca el 'Número de envío' actual");
    println!("(según la aplicación de planilla en línea BAC):");
    let envio: u32 = read!();
    envio
}

pub fn get_month() -> u32 {
    let now = Local::now();
    println!("Hoy es {:?}", now.to_rfc2822());
    let month = now.month();
    println!("Se generaran las planillas para el mes actual ({}):", month);
    month
}

pub fn gen_files(month: u32, envio: u32) {
    let config = config::Config::new(month, envio);
    let employees = employee::get_employees(&config).expect("Error leyendo empleados");
    let mut pay = payment::Payment::new(&config, &employees);
    writepay::write_propina(&config, &mut pay).expect("No pude escribir el achivo pago de propinas");
}
