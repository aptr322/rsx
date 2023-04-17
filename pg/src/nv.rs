#[derive(Debug)]
pub struct NamedVal {
    pub m: String,
    pub v: u32,
}

impl Default for NamedVal {
    fn default() -> NamedVal {
        NamedVal { m: "???".to_string(), v:10 }
    }
}

#[allow(dead_code)]
impl NamedVal {
    pub fn new(name: &str, val: u32) -> Self {
        NamedVal{m: name.into(), v: val}
    }
}

#[allow(dead_code)]
pub fn new_nv() -> NamedVal {
    Default::default()
}
