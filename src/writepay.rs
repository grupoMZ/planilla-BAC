use calamine::{Error};
use std::fs;
use std::path::{PathBuf};
use crate::config::{Config, ConfigError};
use crate::employee::Employee;
use crate::payment::Payment;
use crate::formatprn;


fn compute_payment_fijos_admin(config: &Config, payment: &mut Payment) -> Result<(), ConfigError> {
    let path = config.path.planilla_fijos_dir.clone();
    let mut pathbuf = PathBuf::from(config.path.planilla_fijos_dir.as_str());
    pathbuf.push(&config.path.planilla_fijos);

    let xlpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;
    payment.compute_payment_amount(xlpath, &config.excel.fijos.admin)?;
    Ok(())
}

fn compute_payment_fijos_ops(config: &Config, payment: &mut Payment) -> Result<(), ConfigError> {
    let path = config.path.planilla_fijos_dir.clone();
    let mut pathbuf = PathBuf::from(config.path.planilla_fijos_dir.as_str());
    pathbuf.push(&config.path.planilla_fijos);

    let xlpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;
    payment.compute_payment_amount(xlpath, &config.excel.fijos.ops)?;
    Ok(())
}

fn compute_payment_eventuales_fijos(config: &Config, payment: &mut Payment) -> Result<(), ConfigError> {
    let path = config.path.planilla_eventuales_dir.clone();
    let mut pathbuf = PathBuf::from(config.path.planilla_eventuales_dir.as_str());
    pathbuf.push(&config.path.planilla_eventuales);

    let xlpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;
    payment.compute_payment_amount(xlpath, &config.excel.eventuales.fijos)?;
    Ok(())
}

fn compute_payment_eventuales_ops(config: &Config, payment: &mut Payment) -> Result<(), ConfigError> {
    let path = config.path.planilla_eventuales_dir.clone();
    let mut pathbuf = PathBuf::from(config.path.planilla_eventuales_dir.as_str());
    pathbuf.push(&config.path.planilla_eventuales);

    let xlpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;
    payment.compute_payment_amount(xlpath, &config.excel.eventuales.ops)?;
    Ok(())
}


pub fn write_salario(config: &Config, employees: &Vec<Employee>, payment: &mut Payment) -> Result<(), ConfigError> {

    compute_payment_fijos_admin(config, payment)?;
    compute_payment_fijos_ops(config, payment)?;
    compute_payment_eventuales_fijos(config, payment)?;
    compute_payment_eventuales_ops(config, payment)?;

    let mut contents = formatprn::gen_first_line(config, payment, &config.get_envio_salario());
    let text = &config.bac.texto_salario;
    contents.push_str(formatprn::gen_employee_entries(config, payment, employees, text, 
    &config.get_envio_salario()).as_str());
    let mut pathbuf = PathBuf::from(config.path.pago_bac_dir.as_str());
    pathbuf.push(&config.path.pago_bac_salario);
    let path = config.path.pago_bac_salario.clone();
    let prnpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;
    fs::write(prnpath, contents)?;
    Ok(())
}

pub fn write_viatico(config: &Config, employees: &Vec<Employee>, payment: &mut Payment) -> Result<(), ConfigError> {

    let mut pathbuf = PathBuf::from(config.path.planilla_fijos_dir.as_str());
    pathbuf.push(&config.path.planilla_fijos);

    let path = config.path.planilla_fijos.clone();
    let xlpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;

    payment.compute_payment_amount(xlpath, &config.excel.fijos.viaticos)?;

    let mut contents = formatprn::gen_first_line(config, payment, &config.get_envio_viatico());
    let text = &config.bac.texto_viatico;
    contents.push_str(formatprn::gen_employee_entries(config, payment, employees, text, 
    &config.get_envio_viatico()).as_str());
    let mut pathbuf = PathBuf::from(config.path.pago_bac_dir.as_str());
    pathbuf.push(&config.path.pago_bac_viatico);
    let path = config.path.pago_bac_viatico.clone();
    let prnpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;
    fs::write(prnpath, contents)?;
    Ok(())
}

pub fn write_propina(config: &Config, employees: &Vec<Employee>, payment: &mut Payment) -> Result<(), ConfigError> {

    let mut pathbuf = PathBuf::from(config.path.planilla_eventuales_dir.as_str());
    pathbuf.push(&config.path.planilla_eventuales);

    let path = config.path.planilla_eventuales.clone();
    let xlpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;

    payment.compute_payment_amount(xlpath, &config.excel.eventuales.propina)?;

    let mut contents = formatprn::gen_first_line(config, payment, &config.get_envio_propina());
    let text = &config.bac.texto_propina;
    contents.push_str(formatprn::gen_employee_entries(config, payment, employees, text, 
    &config.get_envio_propina()).as_str());
    let mut pathbuf = PathBuf::from(config.path.pago_bac_dir.as_str());
    pathbuf.push(&config.path.pago_bac_propina);
    let path = config.path.pago_bac_propina.clone();
    let prnpath = pathbuf.as_path().to_str()
        .ok_or_else(|| ConfigError::PathError { path })?;

    fs::write(prnpath, contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn payment_propina() {

    }
}
