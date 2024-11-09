pub struct Hailstone {
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64
}

impl Hailstone {
    pub fn parse(line: &str) -> Hailstone {
        let (s_fields, d_fields) = line.split_once('@').unwrap();
        let s_values: Vec<f64> = s_fields.trim().split(", ").map(|f| f.trim().parse().unwrap()).collect();
        let d_values: Vec<f64> = d_fields.trim().split(", ").map(|f| f.trim().parse().unwrap()).collect();
        
        Hailstone {
            sx: s_values[0],
            sy: s_values[1],
            sz: s_values[2],

            dx: d_values[0],
            dy: d_values[1],
            dz: d_values[2]
        }
    }
}
