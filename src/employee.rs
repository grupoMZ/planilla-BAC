use std::path::{PathBuf};

use calamine::{open_workbook, DataType, Error, RangeDeserializerBuilder, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Employee {
    pub nit: String,
    pub nombre: String,
    pub cuenta: String,
    pub alias: String,
}

pub fn get_employees(config: &Config) -> Result<Vec<Employee>, Error> {
    let mut path = PathBuf::from(config.path.empleados_bac.as_str());
    let ps = path.as_path().to_str();
    if let Some(s) = ps {
        return get_employees_from(s);
    } else {
        return Err(Error::Msg("No pude encontrar el fichero de empleados"));
    }
}

fn get_employees_from(path: &str) -> Result<Vec<Employee>, Error> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut range = workbook
        .worksheet_range("data")
        .ok_or(Error::Msg("No puedo encontrar la hoja 'data'"))??;
    let columns = ["nit", "nombre", "cuenta", "alias"];
    let mut iter_result =
        RangeDeserializerBuilder::with_headers(&columns).from_range::<_, Employee>(&range)?;
    let mut res: Vec<Employee> = Vec::new();
    for result in iter_result {
        res.push(result?);
    }
    /*     for v in res.iter() {
        println!("{:?}", v);
    } */
    Ok((res))
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
