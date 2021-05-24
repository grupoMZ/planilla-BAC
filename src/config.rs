// TODO: Detect month automatically
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Config {
    pub path: Path,
    pub bac: BAC,
    pub excel: Excel,
    pub month: Vec<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Path {
    pub planilla_fijos_dir: String,
    pub planilla_eventuales_dir: String,
    pub pago_bac_dir: String,
    pub empleados_bac: String,
    pub planilla_fijos: String,
    pub planilla_eventuales: String,
    pub pago_bac_salario: String,
    pub pago_bac_propina: String,
    pub pago_bac_viatico: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct BAC {
    pub batch: String,
    pub trans: String,
    pub date: String,
    pub plan: String,
    pub mes: String,
    pub envio: u32,
    pub colwidth: Vec<usize>,
    pub texto_salario: String,
    pub texto_propina: String,
    pub texto_viatico: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Excel {
    pub name: String,
    pub amount: String,
    pub eventuales: Eventuales,
    pub fijos: Fijos,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Eventuales {
    pub fijos: String,
    pub ops: String,
    pub propina: String,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Fijos {
    pub admin: String,
    pub ops: String,
    pub viaticos: String,
}


impl Config {
    pub fn new(date: String, envio: u32) -> Config {
        let mut c = Config::get_config().expect("Error en el archivo de configuración.");
        c.bac.date = date;
        c.replace_month();
        c.replace_envio_correlative(envio);
        c
    }

    fn get_config() -> Result<Config, Box<dyn Error>> {
        let path = "config.json";
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let c = serde_json::from_reader(reader)?;

        Ok(c)
    }

    fn replace_month(&mut self) {
        let month = &self.bac.date[4..6].to_string();  // date in format "YYYYMMDD"
        let m: usize = month.parse().unwrap();
        let idx = m - 1;
        self.path.pago_bac_salario = self.path.pago_bac_salario.replace("%%%", &self.month[idx]);
        self.path.pago_bac_viatico = self.path.pago_bac_viatico.replace("%%%", &self.month[idx]);
        self.path.pago_bac_propina = self.path.pago_bac_propina.replace("%%%", &self.month[idx]);
        self.bac.texto_salario = self.bac.texto_salario.replace("%%%", &self.month[idx]);
        self.bac.texto_viatico = self.bac.texto_viatico.replace("%%%", &self.month[idx]);
        self.bac.texto_propina = self.bac.texto_propina.replace("%%%", &self.month[idx]);
        self.path.planilla_fijos = self.path.planilla_fijos.replace("%%%", &self.month[idx]);
        self.path.planilla_fijos = self.path.planilla_fijos.replace("##", &month);
        self.path.planilla_eventuales = self.path.planilla_eventuales.replace("%%%", &self.month[idx]);
        self.path.planilla_eventuales = self.path.planilla_eventuales.replace("##", &month);
        self.bac.mes = month.clone();
    }

    fn replace_envio_correlative(&mut self, envio: u32) {
        self.bac.envio = envio;
        let es = self.get_envio_salario();
        self.path.pago_bac_salario = self.path.pago_bac_salario.replace("#####", &es);
        self.bac.texto_salario = self.bac.texto_salario.replace("#####", &es);
        let es = self.get_envio_viatico();
        self.path.pago_bac_viatico = self.path.pago_bac_viatico.replace("#####", &es);
        self.bac.texto_viatico = self.bac.texto_viatico.replace("#####", &es);
        let es = self.get_envio_propina();
        self.path.pago_bac_propina = self.path.pago_bac_propina.replace("#####", &es);
        self.bac.texto_propina = self.bac.texto_propina.replace("#####", &es);
    }

    pub fn get_envio_salario(&self) -> String {
        format!("{:0>5}", self.bac.envio)
    }

    pub fn get_envio_viatico(&self) -> String {
        format!("{:0>5}", self.bac.envio+1)
    }

    pub fn get_envio_propina(&self) -> String {
        format!("{:0>5}", self.bac.envio+2)
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_cfg() {
        let c = Config::new("20200101".to_string(), 123);

        assert_eq!("9679", c.bac.plan);
        assert_eq!("DEC", c.month[11]);
        assert_eq!("PROPINA ENE", c.bac.texto_propina);
        assert_eq!("00123 pago BAC salario ENE.prn", c.path.pago_bac_salario);
        assert_eq!("00124 pago BAC viatico ENE.prn", c.path.pago_bac_viatico);
        assert_eq!("00125 pago BAC propina ENE.prn", c.path.pago_bac_propina);
        assert_eq!("01 GMZ Planilla Operaciones ENE.xlsx", c.path.planilla_fijos);
    }

    #[test]
    fn get_envio() {
        let c = Config::new("20200101".to_string(), 123);

        assert_eq!("00123", c.get_envio_salario());
        assert_eq!("00124", c.get_envio_viatico());
        assert_eq!("00125", c.get_envio_propina());
    }
}