use std::collections::HashMap;

use calamine::{open_workbook, DataType, Error, Reader, Xlsx};

use crate::config::Config;
use crate::employee::Employee;

pub struct Payment {
    pub persons: HashMap<String, u64>,
    pub date: String,
    column: Column,
    text: Text,
}

struct Column {
    alias: usize,
    amount: usize,
}

struct Text {
    alias: String,
    amount: String,
}


impl Payment {
    pub fn new(config: &Config, employees: &Vec<Employee>) -> Payment {
        let persons  = HashMap::new();
        let date = config.get_payment_date();
        let column = Column {
            alias: 255,
            amount: 255,
        };
        let text = Text {
            alias: config.excel.name.clone(),
            amount: config.excel.amount.clone(),
        };
        let mut p = Payment {
            persons,
            date,
            column,
            text,
        };
        p.set_persons(employees);
        p
    }

    fn set_persons(&mut self, employees: &Vec<Employee>) {
        for employee in employees.iter() {
            self.persons.insert(employee.alias.clone(), 0);
        }
    }

    #[cfg(test)]
    pub fn new_test_payment() -> Payment {
        let mut persons = HashMap::new();
        persons.insert("Maria Jose".to_string(), 123456 as u64);
        persons.insert("Siria".to_string(), 7890 as u64);
        let payment = Payment {
            persons,
            date: String::from("20210530"),
            column: Column {
                alias: 255,
                amount: 255,
            },
            text: Text {
                alias: "NOMBRE".to_string(),
                amount: "RECIB".to_string(),
            },
        };

        payment
    }

    pub fn get_total_payment(&self) -> u64 {
        let mut total = 0;

        for (_alias, amount ) in self.persons.iter() {
            total += amount;
        }
        total
    }

    pub fn get_total_transactions(&self) -> u64 {
        self.persons.len() as u64
    }

    pub fn compute_payment_amount(&mut self, path: &str, sheet: &String) -> Result<(), Error> {
        println!("Intentando abrir el archivo: '{}'", path);
        let mut workbook: Xlsx<_> = open_workbook(path).expect(format!("No {}", path).as_str());
        println!("Buscando la hoja '{}' en el archivo '{}'",
            &sheet, &path
        );
        let range = workbook
            .worksheet_range(sheet)
            .ok_or(Error::Msg("La hoja no fue encontrada"))??;
        for (_i, row) in range.rows().enumerate() {
            let column = self.find_name_amount_columns(row);
            match column {
                None => continue,
                Some(c) => {
                    self.column.amount = c.amount;
                    self.column.alias = c.alias;
                    break;
                }
            }
        }
        for (i, row) in range.rows().enumerate() {
            self.compute_persons_payment(row, i)?;
        }

        Ok(())
    }

    fn compute_persons_payment(&mut self, row: &[DataType], i: usize) -> Result<(), Error> {
        let col = &row[self.column.alias];
        if col.is_string() {
            let alias: String = col.to_string();
            for (person_alias, amount) in self.persons.iter_mut() {
                if alias.contains(person_alias) {
                    let f = &row[self.column.amount];
                    if f.is_float() {
                        let ff = f.get_float();
                        let n: u64 = match ff {
                            None => {
                                return Err(Error::Msg("Celda vacía"));
                            }
                            Some(nn) => (nn * 100.0).round() as u64,
                        };
                        *amount += n;
                    } else if f.is_int() {
                        let ff = f.get_int();
                        match ff {
                            None => {
                                return Err(Error::Msg("Celda vacia"));
                            }
                            Some(nn) => nn as u64,
                        };
                    } else {
                        println!("Advertencia: Fila:{}; Columna:{}", i+1, self.column.amount);
                        //return Err(Error::Msg("Contenido de la celda no es un número"));
                        return Ok(());
                    }
                }
                //println!("{}: {}", k, col);
            }
        }
        Ok(())
    }

    fn find_name_amount_columns(&self, row: &[DataType]) -> Option<Column> {
        let mut column = Column {
            alias: 255,
            amount: 255,
        };
        for (k, col) in row.iter().enumerate() {
            if col.is_string() {
                let s: String = col.to_string();
                //println!("{}: {}", k, col);
                if s.contains(&self.text.alias) {
                    column.alias = k;
                } else if s.contains(&self.text.amount) {
                    column.amount = k;
                }
                if (column.alias != 255) && (column.amount != 255) {
                    return Some(column);
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::employee::get_employees;
    #[test]
    fn new_payment() {
        let config = Config::new(1, 1);
        let employees = get_employees(&config).expect("Error leyendo empleados");
        let payment = Payment::new(&config, &employees);
        let e0 = &employees[0];
        let last = employees.len() - 1;
        let el = &employees[last];
        assert_eq!(0, payment.persons[&e0.alias]);
        assert_eq!(0, payment.persons[&el.alias]);
    }
    #[test]
    fn get_pay() {
        let path = "../Planilla_ISSS_y_AFP_2021/04 GMZ Planilla Operaciones ABR.xlsx";
        let sheet = " Planilla Ops  1  al 31 ".to_string();
        let mut payment = Payment::new_test_payment();
        payment.compute_payment_amount(path, &sheet).expect("ERROR");
        //assert_matches!(e, u);
        assert_eq!(1, payment.column.alias);
        assert_eq!(22, payment.column.amount);
        assert_eq!(123456, payment.persons["Maria Jose"]); // remains unchanged from test
        assert_eq!(7890 + 32579, payment.persons["Siria"]); // add to test the amount in planilla
    }

    #[test]
    fn total_payment() {
        let payment = Payment::new_test_payment();
        let d = payment.get_total_payment();
        assert_eq!(131346, d);
    }

    #[test]
    fn total_transactions() {
        let payment = Payment::new_test_payment();
        let d = payment.get_total_transactions();
        assert_eq!(2, d);
    }
}
