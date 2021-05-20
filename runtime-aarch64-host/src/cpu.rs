pub struct CpuInfo {
    pub model: CpuModel,
    pub capabilities: Vec<String>,
}

impl CpuInfo {
    pub fn try_new() -> anyhow::Result<CpuInfo> {
        let model = CpuModel {
            vendor: "ARM".to_string(),
            stepping: 0 as u8,
            family: 0 as u16,
            model: 3 as u16
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
        "fp".to_string(),
        "asimd".to_string(),
        "evtstrm".to_string(), 
        "crc32".to_string()
        ];
    Ok(stringvector)
}