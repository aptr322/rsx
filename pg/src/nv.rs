#[derive(Debug)]
pub struct NamedVal {
    m: String,
    v: u32,
}

impl Default for NamedVal {
    fn default() -> NamedVal {
        NamedVal { m: "???".to_string(), v:10 }
    }
}


impl NamedVal {
    pub fn new(name: &str, val: u32) -> Self {
        NamedVal{m: name.into(), v: val}
    }
}

pub fn new_nv() -> NamedVal {
    Default::default()
}
