use serde::export::TryFrom;

pub struct CpuInfo {
    pub model: CpuModel,
    pub capabilities: Vec<String>,
}

impl CpuInfo {
    pub fn try_new() -> anyhow::Result<CpuInfo> {
        let model = String::from("Cortex-A72");
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
//impl<'a> TryFrom<&'a CpuId> for CpuModel {
impl<'a> TryFrom<'a> for CpuModel {
    type Error = anyhow::Error;

    fn try_from() -> Result<Self, Self::Error> {
        Ok(CpuModel {
            vendor:&str = "Licensed by ARM";
            stepping: ("undefined" as u8)
            family: ("r0p3" as u16) + ("" as u16),
            model: (("") as u16) << 4) + ("Raspberry Pi 4" as u16),
        })
    }
}

fn cpu_features() -> anyhow::Result<Vec<String>> {
    let stringvector = vec!["ARM Processor, feature set undefined"];
    Ok(stringvector)
}
