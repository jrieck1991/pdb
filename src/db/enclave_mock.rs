use rand::random;
use sgx_isa::{Attributes, ErrorCode, Miscselect};

// Mock enclave functions to facilitate local development

#[derive(Debug)]
pub struct SealData {
    rand: [u8; 16],
    isvsvn: u16,
    cpusvn: [u8; 16],
    attributes: Attributes,
    miscselect: Miscselect,
}

pub fn seal_key(_label: [u8; 16]) -> ([u8; 16], SealData) {
    let seal_data = SealData {
        rand: random(),

        isvsvn: 5,
        cpusvn: random(),
        attributes: Attributes::default(),
        miscselect: Miscselect::default(),
    };

    (random(), seal_data)
}

pub fn unseal_key(_label: [u8; 16], _seal_data: SealData) -> Result<[u8; 16], ErrorCode> {
    Ok(random())
}
