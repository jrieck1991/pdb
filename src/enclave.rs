use sgx_isa::{Report, Attributes, Miscselect, ErrorCode, Keyname, Keypolicy, Keyrequest};
use rand::random;

// Info about how sealing key was derived.
#[derive(Debug)]
pub struct SealData {
    rand: [u8; 16],
    isvsvn: u16,
    cpusvn: [u8; 16],
    attributes: Attributes,
    miscselect: Miscselect, 
}

// get a key for sealing data
// returned key may be used for AE
// use different labels for different types of data
// SealData should be stored alongsize the ciphertext
pub fn seal_key(label: [u8; 16]) -> ([u8; 16], SealData) {

    // get enclave report
    let report = Report::for_self();

    // form seal data
    let seal_data = SealData {
        rand: random(),

        // copy params of current enclave into SealData
        isvsvn: report.isvsvn,
        cpusvn: report.cpusvn,
        attributes: report.attributes,
        miscselect: report.miscselect,
    };

    // EGETKEY shouldnt error here because we used info from Report::for_self
    (egetkey(label, &seal_data).unwrap(), seal_data)
}

// get a key for unsealing data
// returned key can be used for AD
// pass in same label used to get the sealing key
// pass in the seal_data returned with the label
pub fn unseal_key(label: [u8; 16], seal_data: SealData) -> Result<[u8; 16], ErrorCode> {

    let report = Report::for_self();

    if report.attributes != seal_data.attributes || report.miscselect != seal_data.miscselect {
        return Err(ErrorCode::InvalidAttribute)
    }

    egetkey(label, &seal_data)
}

// derive a sealing key for the current enclave given label and seal data
fn egetkey(label: [u8; 16], seal_data: &SealData) -> Result<[u8; 16], ErrorCode> {

    // form keyid
    let mut keyid = [0; 32];
    {
        let (label_dst, rand_dst) = keyid.split_at_mut(16);
        label_dst.copy_from_slice(&label);
        rand_dst.copy_from_slice(&seal_data.rand);
    }

    // form egetkey request
    Keyrequest {
        keyname: Keyname::Seal as _,
        keypolicy: Keypolicy::MRENCLAVE,
        isvsvn: seal_data.isvsvn,
        cpusvn: seal_data.cpusvn,
        attributemask: [!0; 2],
        keyid: keyid,
        miscmask: !0,
        ..Default::default()
    }.egetkey()
}