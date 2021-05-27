use crate::config::{Config, ConfigError, Output};
use crate::employee::Employee;
use crate::formatprn;
use crate::payment::Payment;
use calamine::Error;
use std::fs;
use std::path::PathBuf;

fn compute_payment(output: &Output, payment: &mut Payment) -> Result<(), ConfigError> {
    payment.reset_amount();
    for input in output.inputs.iter() {
        let path = input.dir.clone();
        let mut pathbuf = PathBuf::from(input.dir.as_str());
        pathbuf.push(&input.file);
        let xlpath = pathbuf
            .as_path()
            .to_str()
            .ok_or_else(|| ConfigError::PathError { path })?;
        for sheet in input.sheets.iter() {
            payment.compute_payment_amount(xlpath, sheet)?;
        }
    }
    Ok(())
}

pub fn write_outputs(
    config: &Config,
    employees: &Vec<Employee>,
    payment: &mut Payment,
) -> Result<(), ConfigError> {
    for (i, output) in config.outputs.iter().enumerate() {
        compute_payment(&output, payment)?;

        let mut contents = formatprn::gen_first_line(config, payment, &config.get_envio(i));
        contents.push_str(
            formatprn::gen_employee_entries(
                config,
                payment,
                employees,
                &output.text,
                &config.get_envio(i),
            )
            .as_str(),
        );

        let mut pathbuf = PathBuf::from(output.dir.as_str());
        pathbuf.push(&output.file);
        let path = output.file.clone();
        let prnpath = pathbuf
            .as_path()
            .to_str()
            .ok_or_else(|| ConfigError::PathError { path })?;
        fs::write(prnpath, contents)?;
        println!("[INFO] Se escribió con éxito el archivo '{}' ", prnpath);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn payment_propina() {}
}
