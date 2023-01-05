#[derive(Default,Debug)]
pub struct NamedVal {
    m: String,
    v: u32,
}

pub fn new_nv() -> NamedVal {
    Default::default()
}
