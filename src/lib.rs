
use calamine::{open_workbook, DataType, Error, RangeDeserializerBuilder, Reader, Xlsx};
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use text_io::read;

mod config;
mod employee;
mod formatprn;
mod payment;

}
