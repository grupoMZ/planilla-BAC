// TODO: Detect month automatically
use calamine::{DeError, XlsxError};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{err:#?} \r\nEl archivo {path:?} no fue encontrado.")]
    FileNameError { err: std::io::Error, path: String },
    #[error("{err:#?} \r\nEl archivo {path:?} no fue encontrado.")]
    ExcelFileError { err: XlsxError, path: String },
    #[error("La hoja {sheet:?} no fue encontrada en el archivo {fname:?}.")]
    ExcelSheetError { sheet: String, fname: String },
    #[error("{err:#?} \r\nLa hoja {sheet:?} en el archivo {fname:?} no pudo ser analizada.")]
    ExcelParseError {
        err: DeError,
        sheet: String,
        fname: String,
    },
    #[error("La ruta {path:?} no es valida.")]
    PathError { path: String },
    #[error("{err:#?} \r\nEl archivo {path:?} no pudo ser analizado.")]
    ParseError {
        err: serde_json::Error,
        path: String,
    },
    #[error("En archivo {fname:?}, hoja {sheet:?},
no fue encontrada una fila con los subtextos en celda según archivo de configuración:
\"excel.amount\": {text_amount:?},\r\n\"excel.name\": {text_alias:?}.")]
    ExcelTextError {
        fname: String,
        sheet: String,
        text_amount: String,
        text_alias: String,
    },
    #[error("Error en la celda (fila: {row:?}, columna: {col:?}).")]
    ExcelCellError { row: usize, col: usize },
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Closing")]
    EndError,
}
#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Config {
    pub outputs: Vec<Output>,
    pub bac: BAC,
    pub excel: Excel,
    pub month: Vec<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Output {
    pub dir: String,
    pub file: String,
    pub text: String,
    pub inputs: Vec<Input>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Input {
    pub dir: String,
    pub file: String,
    pub sheets: Vec<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct BAC {
    pub batch: String,
    pub trans: String,
    pub date: String,
    pub plan: String,
    pub envio: u32,
    pub colwidth: Vec<usize>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Excel {
    pub name: String,
    pub amount: String,
    pub empleados_bac: String,
}

impl Config {
    pub fn new(date: String, month_in: String, 
        envio: u32, config: String) -> Result<Config, ConfigError> {
        let mut c = Config::get_config(config)?;
        c.bac.date = date;
        c.replace_month(&month_in);
        c.replace_envio_correlative(envio);
        Ok(c)
    }

    fn get_config(path: String) -> Result<Config, ConfigError> {
        let path_ = path.clone();
        let file = File::open(&path_).map_err(|err| ConfigError::FileNameError { err, path })?;
        let reader = BufReader::new(file);

        let c = serde_json::from_reader(reader)
            .map_err(|err| ConfigError::ParseError { err, path: path_ })?;

        Ok(c)
    }

    fn replace_month(&mut self, month_in: &String) {
        let m: usize = month_in.parse().unwrap();
        let idx = m - 1;
        for output in self.outputs.iter_mut() {
            output.text = output.text.replace("%%%", &self.month[idx]);
            output.file = output.file.replace("%%%", &self.month[idx]);
            for input in output.inputs.iter_mut() {
                input.file = input.file.replace("%%%", &self.month[idx]);
                input.file = input.file.replace("##", &month_in);
            }
        }
    }

    fn replace_envio_correlative(&mut self, envio: u32) {
        self.bac.envio = envio;
        let mut envios = Vec::new();
        for i in 0..self.outputs.len() {
            envios.push(self.get_envio(i));
        }
        for (i, output) in self.outputs.iter_mut().enumerate() {
            output.file = output.file.replace("#####", &envios[i]);
        }
    }

    pub fn get_envio(&self, offset: usize) -> String {
        format!("{:0>5}", self.bac.envio + offset as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_cfg() {
        let c = Config::new("20200101".to_string(), "01".to_string(), 
        123, "config.json".to_string()).unwrap();

        assert_eq!("9679", c.bac.plan);
        assert_eq!("DEC", c.month[11]);
        assert_eq!("PROPINA ENE", c.outputs[2].text);
        assert_eq!("00123 pago BAC salario ENE.prn", c.outputs[0].file);
        assert_eq!("00124 pago BAC viatico ENE.prn", c.outputs[1].file);
        assert_eq!("00125 pago BAC propina ENE.prn", c.outputs[2].file);
        assert_eq!(
            "01 GMZ Planilla Operaciones ENE.xlsx",
            c.outputs[0].inputs[0].file
        );
    }

    #[test]
    fn get_envio() {
        let c = Config::new("20200101".to_string(), "01".to_string(),
         123, "config.json".to_string()).unwrap();

        assert_eq!("00123", c.get_envio(0));
        assert_eq!("00125", c.get_envio(2));
    }
}
