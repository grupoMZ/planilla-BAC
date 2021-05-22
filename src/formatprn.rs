use crate::config::Config;
use crate::payment as pay;
use crate::payment::Payment;

fn gen_first_line(config: &Config, payment: &Payment) -> String {
    let colwidth = &config.bac.colwidth;
    let bac = &config.bac;
    let mut strlen: u8 = 0;
    for v in colwidth.iter() {
        strlen += v;
    }
    let mut s = String::with_capacity(strlen as usize);
    s.push_str(&bac.batch);
    s.push_str(&bac.plan);
    s.push_str(&bac.envio);
    //let f = format!("{:>width$}", " ", width=colwidth[3] as usize);
    s.push_str(format!("{:>width$}", " ", width = colwidth[3] as usize).as_str());
    s.push_str(format!("{:>width$}", " ", width = colwidth[4] as usize).as_str());
    s.push_str(config.get_payment_date().as_str());
    s.push_str(
        format!(
            "{: >width$}",
            payment.get_total_payment(),
            width = colwidth[8] as usize
        )
        .as_str(),
    );
    s.push_str(
        format!(
            "{: >width$}",
            payment.get_total_transactions(),
            width = colwidth[9] as usize
        )
        .as_str(),
    );

    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_line() {
        let mut c = Config::new(5, 123);
        c.bac.envio = String::from("00019");
        let payment = Payment::new_test_payment();
        let s = gen_first_line(&c, &payment);
        assert_eq!(
            "B967900019                         20210530       131346    2",
            s.as_str()
        );
    }
}
