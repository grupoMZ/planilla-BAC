use calamine::{open_workbook, DataType, Error, RangeDeserializerBuilder, Reader, Xlsx};
use std::fs;

use crate::config::Config;
use crate::employee::{get_employees, Employee};

pub struct Payment {
    pub persons: Vec<Person>,
    date: String,
    column: Column,
    text: Text,
}

pub struct Person {
    pub alias: String,
    pub amount: u64,
}

struct Column {
    alias: usize,
    amount: usize,
}

struct Text {
    alias: String,
    amount: String,
}

impl Person {
    pub fn new(employee: &Employee) -> Person {
        Person {
            alias: employee.alias.clone(),
            amount: 0,
        }
    }
}
impl Payment {
    pub fn new(config: &Config, employees: &Vec<Employee>) -> Payment {
        let persons: Vec<Person> = Vec::new();
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
            self.persons.push(Person::new(employee));
        }
    }

    pub fn write_payments(
        &self,
        config: &Config,
        employees: &Vec<Employee>,
    ) -> std::io::Result<()> {
        let text = format!("{}", 0);
        let path = String::from(""); // TODO: compute the path
        fs::write(path, text)
    }

    pub fn new_test_payment() -> Payment {
        let payment = Payment {
            persons: vec![
                Person {
                    alias: "Maria Jose".to_string(),
                    amount: 123456,
                },
                Person {
                    alias: "Siria".to_string(),
                    amount: 7890,
                },
            ],
            date: String::from("20210519"),
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

        for person in self.persons.iter() {
            total += person.amount;
        }
        total
    }

    pub fn get_total_transactions(&self) -> u64 {
        let mut total = 0;

        for person in self.persons.iter() {
            total += 1;
        }
        total
    }

    pub fn compute_payment_amount(&mut self, path: String, sheet: String) -> Result<(), Error> {
        let mut workbook: Xlsx<_> = open_workbook(&path)?;
        let errmsg = format!(
            "La hoja {} no fue encontrada en el archivo {}",
            &sheet, &path
        );
        let mut range = workbook
            .worksheet_range(&sheet)
            .ok_or(Error::Msg("La hoja no fue encontrada"))??;
        for (i, row) in range.rows().enumerate() {
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
            self.compute_persons_payment(row)?;
        }

        Ok(())
    }

    fn compute_persons_payment(&mut self, row: &[DataType]) -> Result<(), Error> {
        let col = &row[self.column.alias];
        if col.is_string() {
            let alias: String = col.to_string();
            for person in self.persons.iter_mut() {
                if alias.contains(&person.alias) {
                    let f = &row[self.column.amount];
                    if f.is_float() {
                        let ff = f.get_float();
                        let n: u64 = match ff {
                            None => {
                                return Err(Error::Msg("ABC"));
                            }
                            Some(nn) => (nn * 100.0).round() as u64,
                        };
                        person.amount += n;
                    } else {
                        return Err(Error::Msg("Not a float"));
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

pub fn write_propina(payment: &mut Payment, employees: Vec<Employee>) {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn write_file() {
        let config = Config::new(1, 0);
        let path = "./src/00000 Pago BAC Propina ENE.prn".to_string();
        let mut payment = Payment::new_test_payment();
        let employees = get_employees(&config).expect("Error opening employees");

        payment.write_payments(&config, &employees);
    }

    #[test]
    fn new_payment() {
        let mut config = Config::new(1, 1);
        let employees = get_employees(&config).expect("Error leyendo empleados");
        let payment = Payment::new(&config, &employees);
        let p0 = &payment.persons[0];
        let e0 = &employees[0];
        let last = employees.len() - 1;
        let pl = &payment.persons[last];
        let el = &employees[last];
        assert_eq!(&e0.alias, &p0.alias);
        assert_eq!(0, p0.amount);
        assert_eq!(&el.alias, &pl.alias);
        assert_eq!(0, pl.amount);
    }
    #[test]
    fn get_pay() {
        let path = "../Planilla_ISSS_y_AFP_2021/04 GMZ Planilla Operaciones ABR.xlsx".to_string();
        let sheet = " Planilla Ops  1  al 31 ".to_string();
        let mut payment = Payment::new_test_payment();
        let e = payment.compute_payment_amount(path, sheet).expect("ERROR");
        //assert_matches!(e, u);
        assert_eq!(1, payment.column.alias);
        assert_eq!(22, payment.column.amount);
        assert_eq!(123456, payment.persons[0].amount); // remains unchanged from test
        assert_eq!(7890 + 32579, payment.persons[1].amount); // add to test the amount in planilla
    }

    #[test]
    fn total_payment() {
        let mut payment = Payment::new_test_payment();
        let d = payment.get_total_payment();
        assert_eq!(131346, d);
    }

    #[test]
    fn total_transactions() {
        let mut payment = Payment::new_test_payment();
        let d = payment.get_total_transactions();
        assert_eq!(2, d);
    }
}
