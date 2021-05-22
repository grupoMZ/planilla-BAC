/* Interesting rows
* Pago BAC Salario:
*   Planilla Operaciones -> Planilla Ops 1 al 31: Col. B (NOMBRE EMPLEADO ), Col. W (NETO A RECIBIR)
*   Planilla Operaciones -> Planilla Admin: Col. B (NOMBRE EMPLEADO ), Col. U (NETO A RECIBIR)
*   Planilla de Eventuales -> Planilla Fija Operacion: Col. C (NOMBRE), Col. W (RECIBE)
* Pago BAC viatico
*   Planilla Operaciones -> Viaticos OPs: Col. C (NOMBRE), Col. O (RECIBE)
* Pago BAC propina
*   Planilla de Eventuales -> Propina Bar y Cocina: Col. C (NOMBRE), Col. J (RECIBE)
*/

use calamine::{open_workbook, DataType, Error, RangeDeserializerBuilder, Reader, Xlsx};
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use text_io::read;

mod config;
mod employee;
mod formatprn;
mod payment;

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
    let mut config = config::Config::new(month, envio);
    let employees = employee::get_employees(&config).expect("Error leyendo empleados");
    let pay = payment::Payment::new(&config, &employees);
    pay.write_payments(&config, &employees).expect("No pude escribir los archivos");
}
