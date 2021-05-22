// TODO: Detect month automatically
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Config {
    pub path: path,
    pub bac: bac,
    pub excel: excel,
    pub month: Vec<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct path {
    planilla_fijos_dir: String,
    planilla_eventuales_dir: String,
    pub empleados_bac: String,
    planilla_fijos: String,
    planilla_eventuales: String,
    pago_bac_salario: String,
    pago_bac_propina: String,
    pago_bac_viatico: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct bac {
    pub batch: String,
    pub trans: String,
    pub plan: String,
    pub envio: String,
    pub mes: String,
    pub colwidth: Vec<u8>,
    pub texto_salario: String,
    pub texto_propina: String,
    pub texto_viatico: String,
    pub dia_pago: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct excel {
    pub name: String,
    pub amount: String,
    eventuales: eventuales,
    fijos: fijos,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct eventuales {
    fijos: String,
    propina: String,
    viaticos: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct fijos {
    admin: String,
    ops: String,
    viaticos: String,
}

impl Config {
    pub fn new(month: u32, envio: u32) -> Config {
        let mut c = Config::get_config().expect("Error en el archivo de configuración.");
        Config::replace_month(&mut c, month);
        Config::replace_envio_correlative(&mut c, envio);
        c
    }

    pub fn get_payment_date(&self) -> String {
        let now = Local::now();
        let d = &self.bac.dia_pago;
        let m = &self.bac.mes;
        format!("{}{:02}{:02}", now.year(), m, d)
    }

    fn get_config() -> Result<Config, Box<dyn Error>> {
        let path = "config.json";
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let c = serde_json::from_reader(reader)?;

        Ok(c)
    }

    fn replace_month(c: &mut Config, month: u32) {
        let ms = format!("{:0>2}", month);
        let idx = (month as usize) - 1;
        c.path.pago_bac_salario = c.path.pago_bac_salario.replace("%%%", &c.month[idx]);
        c.path.pago_bac_viatico = c.path.pago_bac_viatico.replace("%%%", &c.month[idx]);
        c.path.pago_bac_propina = c.path.pago_bac_propina.replace("%%%", &c.month[idx]);
        c.bac.texto_propina = c.bac.texto_propina.replace("%%%", &c.month[idx]);
        c.path.planilla_fijos = c.path.planilla_fijos.replace("%%%", &c.month[idx]);
        c.path.planilla_fijos = c.path.planilla_fijos.replace("##", &ms);
        c.bac.mes = ms.clone();
    }

    fn replace_envio_correlative(config: &mut Config, envio: u32) {
        let mut e = envio;
        let es = format!("{:0>5}", e);
        config.path.pago_bac_salario = config.path.pago_bac_salario.replace("#####", &es);
        config.bac.texto_salario = config.bac.texto_salario.replace("#####", &es);
        e += 1;
        let es = format!("{:0>5}", e);
        config.path.pago_bac_viatico = config.path.pago_bac_viatico.replace("#####", &es);
        config.bac.texto_viatico = config.bac.texto_viatico.replace("#####", &es);
        e += 1;
        let es = format!("{:0>5}", e);
        config.path.pago_bac_propina = config.path.pago_bac_propina.replace("#####", &es);
        config.bac.texto_propina = config.bac.texto_propina.replace("#####", &es);
    }

}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_cfg() {
        let c = Config::new(1, 123);

        assert_eq!("9679", c.bac.plan);
        assert_eq!("DEC", c.month[11]);
        assert_eq!("PROPINA ENE", c.bac.texto_propina);
        assert_eq!("00123 pago BAC salario ENE.prn", c.path.pago_bac_salario);
        assert_eq!("00124 pago BAC viatico ENE.prn", c.path.pago_bac_viatico);
        assert_eq!("00125 pago BAC propina ENE.prn", c.path.pago_bac_propina);
        assert_eq!("01 GMZ Planilla Operaciones ENE.xlsx", c.path.planilla_fijos);
    }

}