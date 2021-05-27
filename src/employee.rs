use std::path::PathBuf;

use crate::config::{Config, ConfigError};
use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Employee {
    pub alias: String,
    pub nombre: String,
    pub nit: String,
    pub cuenta: String,
}

pub fn get_employees(config: &Config) -> Result<Vec<Employee>, ConfigError> {
    let path = config.excel.empleados_bac.clone();
    let pbuf = PathBuf::from(config.excel.empleados_bac.as_str());
    let xlpath = pbuf
        .as_path()
        .to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;
    return get_employees_from(xlpath);
}

fn get_employees_from(xlpath: &str) -> Result<Vec<Employee>, ConfigError> {
    let path = String::from(xlpath);
    let mut workbook: Xlsx<_> = open_workbook(xlpath)
    .map_err(|err| ConfigError::ExcelFileError { err, path })?;
    let name = "data";
    let sheet = String::from(name);
    let fname = String::from(xlpath);
    let range = workbook
        .worksheet_range("data")
        .ok_or_else(|| ConfigError::ExcelSheetError { sheet, fname })?
        .map_err(|err| ConfigError::ExcelFileError {
            err,
            path: String::from(xlpath),
        })?;

    let sheet = String::from(name);
    let fname = String::from(xlpath);
    let columns = ["alias", "nombre", "nit", "cuenta"];
    let iter_result = RangeDeserializerBuilder::with_headers(&columns)
        .from_range::<_, Employee>(&range)
        .map_err(|err| ConfigError::ExcelParseError { err, sheet, fname })?;

    let mut res: Vec<Employee> = Vec::new();
    for result in iter_result {
        let sheet = String::from(name);
        let fname = String::from(xlpath);
        let employee = result.map_err(|err| ConfigError::ExcelParseError { err, sheet, fname })?;
        res.push(employee);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let path = "./src/test_empleados_bac.xlsx";
        let e = get_employees_from(path).expect("ERROR");
        let e0 = Employee {
            nit: String::from("123"),
            nombre: String::from("ABC XYZ"),
            cuenta: String::from("789"),
            alias: String::from("Abc"),
        };
        let e1 = Employee {
            nit: String::from("987654321"),
            nombre: String::from("UVW DEF"),
            cuenta: String::from("123"),
            alias: String::from("Def"),
        };
        let u = vec![e0, e1];
        assert_eq!(e, u);
    }
}
