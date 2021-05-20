// use serde::export::TryFrom;

pub struct CpuInfo {
    pub model: CpuModel,
    pub capabilities: Vec<String>,
}

impl CpuInfo {
    pub fn try_new() -> anyhow::Result<CpuInfo> {
        let model = CpuModel {
            vendor: "GenuineIntel".to_string(),
            stepping: 12 as u8,
            family: 6 as u16,
            model: 158 as u16
        };

        let capabilities = cpu_features()?;

        Ok(CpuInfo {
            model,
            capabilities,
        })
    }
}
//  Cortex-A72 (ARM v8) 64-bit SoC @ 1.5GHz
pub struct CpuModel {
    pub vendor: String, 
    pub stepping: u8, 
    pub family: u16,  
    pub model: u16, 
}
fn cpu_features() -> anyhow::Result<Vec<String>> {
    let stringvector = vec![
        "sse3".to_string(),
        "pclmulqdq".to_string(),
        "dtes64".to_string(),
        "monitor".to_string(),
        "dscpl".to_string(),
        "vmx".to_string(),
        "smx".to_string(),
        "eist".to_string(),
        "tm2".to_string(),
        "ssse3".to_string(),
        "fma".to_string(),
        "cmpxchg16b".to_string(),
        "pdcm".to_string(),
        "pcid".to_string(),
        "sse41".to_string(),
        "sse42".to_string(),
        "x2apic".to_string(),
        "movbe".to_string(),
        "popcnt".to_string(),
        "tsc_deadline".to_string(),
        "aesni".to_string(),
        "xsave".to_string(),
        "osxsave".to_string(),
        "avx".to_string(),
        "f16c".to_string(),
        "rdrand".to_string(),
        "fpu".to_string(),
        "vme".to_string(),
        "de".to_string(),
        "pse".to_string(),
        "tsc".to_string(),
        "msr".to_string(),
        "pae".to_string(),
        "mce".to_string(),
        "cx8".to_string(),
        "apic".to_string(),
        "sep".to_string(),
        "mtrr".to_string(),
        "pge".to_string(),
        "mca".to_string(),
        "cmov".to_string(),
        "pat".to_string(),
        "pse36".to_string(),
        "clfsh".to_string(),
        "ds".to_string(),
        "acpi".to_string(),
        "mmx".to_string(),
        "fxsr".to_string(),
        "sse".to_string(),
        "sse2".to_string(),
        "ss".to_string(),
        "htt".to_string(),
        "tm".to_string(),
        "pbe".to_string(),
        "fsgsbase".to_string(),
        "adjust_msr".to_string(),
        "smep".to_string(),
        "rep_movsb_stosb".to_string(),
        "invpcid".to_string(),
        "deprecate_fpu_cs_ds".to_string(),
        "mpx".to_string(),
        "rdseed".to_string(),
        "rdseed".to_string(),
        "adx".to_string(),
        "smap".to_string(),
        "clflushopt".to_string(),
        "processor_trace".to_string(),
        "sgx".to_string(),
        "sgx_lc".to_string(),
        ];
    Ok(stringvector)
}