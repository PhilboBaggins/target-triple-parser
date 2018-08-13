
pub struct TargetTriple {
    pub architecture: String,
    pub vendor: String,
    pub system: String,
    pub abi: Option<String>,
}

impl TargetTriple {
    pub fn new(architecture: String, vendor: String, system: String, abi: Option<String>) -> TargetTriple {
        TargetTriple {
            architecture: architecture,
            vendor: vendor,
            system: system,
            abi: abi,
        }
    }
}

pub fn parse(s: &str) -> Option<TargetTriple> {
    let parts = s.split("-").collect::<Vec<&str>>();
    //let parts = s.split("-")
    //    .into_iter()
    //    .map(|part| { part.to_string() })
    //    .collect::<Vec<String>>();
    match parts.len() {
        4 => Some(TargetTriple::new(parts.get(0).unwrap().to_string(), parts.get(1).unwrap().to_string(), parts.get(2).unwrap().to_string(), Some(parts.get(3).unwrap().to_string()))),
        3 => Some(TargetTriple::new(parts.get(0).unwrap().to_string(), parts.get(1).unwrap().to_string(), parts.get(2).unwrap().to_string(), None)),
        _ => None,
    }
}
