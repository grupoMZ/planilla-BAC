use crate::config::Config;
use crate::payment::Payment;
use crate::employee::Employee;

pub fn gen_first_line(config: &Config, payment: &Payment, envio: &String) -> String {
    let colwidth = &config.bac.colwidth;
    let bac = &config.bac;
    let mut strlen: usize = 0;
    for v in colwidth.iter() {
        strlen += v;
    }
    let mut s = String::with_capacity(strlen);
    s.push_str(&bac.batch);
    s.push_str(&bac.plan);
    s.push_str(envio);
    //let f = format!("{:>width$}", " ", width=colwidth[3]);
    s.push_str(format!("{:>width$}", " ", width = colwidth[3]).as_str());
    s.push_str(format!("{:>width$}", " ", width = colwidth[4]).as_str());
    s.push_str(payment.date.as_str());
    s.push_str(
        format!(
            "{: >width$}",
            payment.get_total_payment(),
            width = colwidth[8]
        )
        .as_str(),
    );
    s.push_str(
        format!(
            "{: >width$}",
            payment.get_total_transactions(),
            width = colwidth[9]
        )
        .as_str(),
    );
    s.push_str("\r\n");

    s
}

pub fn gen_employee_entries(config: &Config, payment: &Payment, employees: &Vec<Employee>, text: &String, envio: &String) -> String {
    let colwidth = &config.bac.colwidth;
    let bac = &config.bac;
    let mut strlen: usize = 0;
    for v in colwidth.iter() {
        strlen += v;
    }
    let cap = (payment.get_total_transactions() * strlen as u64) as usize;

    let mut s = String::with_capacity(cap);
    for (i, employee) in employees.iter().enumerate() {
    s.push_str(&bac.trans);
    s.push_str(&bac.plan);
    s.push_str(envio);

//    let id = format!("{:0>width$}", &employee.id, width=14);  // always 14 digits
    let id = format_id(&employee.id);
    s.push_str(format!("{:<width$}", id, width = colwidth[3]).as_str());
    s.push_str(format!("{:>width$}", i+1, width = colwidth[4]).as_str());
    s.push_str(payment.date.as_str());
    let amount = payment.persons[&employee.alias];
    s.push_str(format!("{: >width$}", amount, width = colwidth[8]).as_str());
    s.push_str(format!("{:>width$}", " ", width = colwidth[9]).as_str());
    s.push_str(format!("{:<width$}", text, width = colwidth[10]).as_str());
    s.push_str(format!("{:>width$}", " ", width = colwidth[11]).as_str());
    let w = colwidth[12];
    let mut name = employee.nombre.clone();
    name.truncate(w);
    s.push_str(format!("{:<width$}", name, width = w).as_str());
    s.push_str(format!("{:<width$}", employee.cuenta, width = colwidth[13]).as_str());
    s.push_str("\r\n");

    }

    s
}

fn format_id(id: &String) -> String {
    let dui_len = 9;
    let nit_len = 14;
    if id.len() <= dui_len {
        return format!("{:0>width$}", id, width=dui_len);  // always 14 digits
    } else if id.len() <= nit_len {  // ID is a NIT
        return format!("{:0>width$}", id, width=nit_len);  // always 14 digits
    } else {
        return format!("-ID MUY LARGO-");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::employee::get_employees;
    #[test]
    fn first_line_header() {
        let c = Config::new("20210531".to_string(), 17).unwrap();
        let payment = Payment::new_test_payment();
        let s = gen_first_line(&c, &payment, &c.get_envio(0));
        assert_eq!(
            "B967900017                         20210530       131346    2\r\n",
            s.as_str()
        );
    }

    #[test]
    fn second_line_id_nit() {
        let config = Config::new("20210530".to_string(), 17).unwrap();
        let employees = get_employees(&config).expect("Error opening employees");
        let payment = Payment::new(&config, &employees);
        let num = 2;
        let s = gen_employee_entries(&config, &payment, &employees, &config.outputs[num].text, 
        &config.get_envio(num));
        let mut lines = s.lines();
        if let Some(ss) = lines.next() {
        assert_eq!(
            "T96790001905051008991016          120210530            0     PROPINA MAY                    SILVIA ELIZABETH AVENDANO PINE118543040",
           ss);
        } else {
            assert_eq!(true, false);
        };

        }

    #[test]
    fn third_line_id_dui() {
        let config = Config::new("20210530".to_string(), 17).unwrap();
        let employees = get_employees(&config).expect("Error opening employees");
        let payment = Payment::new(&config, &employees);
        let num = 2;
        let s = gen_employee_entries(&config, &payment, &employees, &config.outputs[num].text, 
        &config.get_envio(num));
        let mut lines = s.lines();
        lines.next();
        if let Some(ss) = lines.next() {
        assert_eq!(
            "T967900019055544433               220210530            0     PROPINA MAY                    BRENDA GRISELDA ROMERO HERNAND122641731",
           ss);
        } else {
            assert_eq!(true, false);
        };

        }
    
    #[test]
    fn format_id_nit_short() {
        assert_eq!("00001234567890", format_id(&"1234567890".to_string()));
    }

    #[test]
    fn format_id_nit_exact() {
        assert_eq!("12345678901234", format_id(&"12345678901234".to_string()));
    }

    #[test]
    fn format_id_dui_short() {
        assert_eq!("012345678", format_id(&"12345678".to_string()));
    }

    #[test]
    fn format_id_dui_exact() {
        assert_eq!("123456789", format_id(&"123456789".to_string()));
    }

    #[test]
    fn format_id_too_long() {
        assert_eq!("-ID MUY LARGO-", format_id(&"1234567890123456".to_string()));
    }
    }
