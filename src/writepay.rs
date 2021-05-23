use calamine::{Error};
use std::fs;
use std::path::{PathBuf};
use crate::config::Config;
use crate::employee::Employee;
use crate::payment::Payment;
use crate::formatprn;


fn compute_payment_fijos_admin(config: &Config, payment: &mut Payment) -> Result<(), Error> {
    let mut pathbuf = PathBuf::from(config.path.planilla_fijos_dir.as_str());
    pathbuf.push(&config.path.planilla_fijos);

    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        payment.compute_payment_amount(path, &config.excel.fijos.admin)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de saliarios XXX"));
    }
    Ok(())
}

fn compute_payment_fijos_ops(config: &Config, payment: &mut Payment) -> Result<(), Error> {
    let mut pathbuf = PathBuf::from(config.path.planilla_fijos_dir.as_str());
    pathbuf.push(&config.path.planilla_fijos);

    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        payment.compute_payment_amount(path, &config.excel.fijos.ops)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de saliarios XXX"));
    }
    Ok(())
}

fn compute_payment_eventuales_fijos(config: &Config, payment: &mut Payment) -> Result<(), Error> {
    let mut pathbuf = PathBuf::from(config.path.planilla_eventuales_dir.as_str());
    pathbuf.push(&config.path.planilla_eventuales);

    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        payment.compute_payment_amount(path, &config.excel.eventuales.fijos)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de salarios XXX"));
    }
    Ok(())
}

fn compute_payment_eventuales_ops(config: &Config, payment: &mut Payment) -> Result<(), Error> {
    let mut pathbuf = PathBuf::from(config.path.planilla_eventuales_dir.as_str());
    pathbuf.push(&config.path.planilla_eventuales);

    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        payment.compute_payment_amount(path, &config.excel.eventuales.ops)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de salarios XXX"));
    }
    Ok(())
}


pub fn write_salario(config: &Config, employees: &Vec<Employee>, payment: &mut Payment) -> Result<(), Error> {

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
    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        fs::write(path, contents)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de salarios XXX"));
    }
    Ok(())
}

pub fn write_viatico(config: &Config, employees: &Vec<Employee>, payment: &mut Payment) -> Result<(), Error> {

    let mut pathbuf = PathBuf::from(config.path.planilla_fijos_dir.as_str());
    pathbuf.push(&config.path.planilla_fijos);

    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        payment.compute_payment_amount(path, &config.excel.fijos.viaticos)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de viaticos XXX"));
    }

    let mut contents = formatprn::gen_first_line(config, payment, &config.get_envio_viatico());
    let text = &config.bac.texto_viatico;
    contents.push_str(formatprn::gen_employee_entries(config, payment, employees, text, 
    &config.get_envio_viatico()).as_str());
    let mut pathbuf = PathBuf::from(config.path.pago_bac_dir.as_str());
    pathbuf.push(&config.path.pago_bac_viatico);
    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        fs::write(path, contents)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de viaticos XXX"));
    }
    Ok(())
}

pub fn write_propina(config: &Config, employees: &Vec<Employee>, payment: &mut Payment) -> Result<(), Error> {

    let mut pathbuf = PathBuf::from(config.path.planilla_eventuales_dir.as_str());
    pathbuf.push(&config.path.planilla_eventuales);

    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        payment.compute_payment_amount(path, &config.excel.eventuales.propina)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de propinas XXX"));
    }

    let mut contents = formatprn::gen_first_line(config, payment, &config.get_envio_propina());
    let text = &config.bac.texto_propina;
    contents.push_str(formatprn::gen_employee_entries(config, payment, employees, text, 
    &config.get_envio_propina()).as_str());
    let mut pathbuf = PathBuf::from(config.path.pago_bac_dir.as_str());
    pathbuf.push(&config.path.pago_bac_propina);
    let optpath = pathbuf.as_path().to_str();
    if let Some(path) = optpath {
        fs::write(path, contents)?;
    } else {
        return Err(Error::Msg("No pude crear la ruta al archivo de propinas XXX"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn payment_propina() {

    }
}
