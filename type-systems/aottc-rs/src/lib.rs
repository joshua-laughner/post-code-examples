mod mismatched_types {
    pub fn read_temperature(temperature_file: &str) -> Vec<f64> {
        if let Ok(vals) = read_txt_file(temperature_file) {
            return vals;
        } else {
            return "0.0, 0.0";
        }
    }

    fn read_txt_file(p: &str) -> Result<Vec<f64>, String> {
        todo!()
    }
}




mod pres_one {
    pub fn read_pressure(pressure_file: &str) -> Vec<f64> {
        todo!()
    }

    pub fn demo_pressure() {
        read_pressure("pressure.txt");
    }
}

mod pres_two {
    
    pub fn read_pressure(pressure_file: &str, units_out: &str) -> Vec<f64> {
        todo!()
    }

    pub fn demo_pressure() {
        read_pressure("pressure.txt");
    }
}

mod mistyped {
    pub fn read_rh(rh_file: &str) -> Vec<f64> {
        todo!()
    }

    pub fn demo_relative_humidity() {
        read_relative_humidity("humidity.txt");
    }
}
