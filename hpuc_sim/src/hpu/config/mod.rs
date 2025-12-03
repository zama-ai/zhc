use serde::{Deserialize, Serialize};
use toml;

use crate::{Cycle, MHz};

use super::Policy;

fn default_freq() -> MHz {
    MHz(400.)
}
fn default_fifo_size() -> usize {
    8
}
fn default_isc_query_period() -> usize {
    12
}
fn default_pbs_timeout() -> usize {
    100_000
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct NttParams {
    total_pbs_nb: usize,
    batch_pbs_nb: usize,
    min_pbs_nb: usize,
    radix: usize,
    psi: usize,
    ct_width: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IscParams {
    depth: usize,
    #[serde(default = "default_isc_query_period")]
    query_period: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RegfParams {
    coef_nb: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PbsParams {
    polynomial_size: usize,
    glwe_dimension: usize,
    lwe_dimension: usize,
    pbs_level: usize,
    #[serde(default = "default_pbs_timeout")]
    timeout: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PcParams {
    pem_pc: usize,
    pem_bytes_w: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct KsParams {
    lbx: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PhysicalConfig {
    #[serde(default = "default_freq")]
    freq: MHz,
    #[serde(default = "default_fifo_size")]
    fifo_size: usize,
    ntt_params: NttParams,
    isc_params: IscParams,
    regf_params: RegfParams,
    pbs_params: PbsParams,
    pc_params: PcParams,
    ks_params: KsParams,
}

impl PhysicalConfig {
    pub fn gaussian_44b() -> Self {
        let toml_string = include_str!("gaussian_44b.toml");
        toml::from_str(toml_string).unwrap()
    }

    pub fn gaussian_44b_fast() -> Self {
        let toml_string = include_str!("gaussian_44b_fast.toml");
        toml::from_str(toml_string).unwrap()
    }

    pub fn gaussian_64b() -> Self {
        let toml_string = include_str!("gaussian_64b.toml");
        toml::from_str(toml_string).unwrap()
    }

    pub fn gaussian_64b_fast() -> Self {
        let toml_string = include_str!("gaussian_64b_fast.toml");
        toml::from_str(toml_string).unwrap()
    }

    pub fn gaussian_64b_pfail64() -> Self {
        let toml_string = include_str!("gaussian_64b_pfail64.toml");
        toml::from_str(toml_string).unwrap()
    }

    pub fn gaussian_64b_pfail64_psi64() -> Self {
        let toml_string = include_str!("gaussian_64b_pfail64_psi64.toml");
        toml::from_str(toml_string).unwrap()
    }

    pub fn tuniform_64b_fast() -> Self {
        let toml_string = include_str!("tuniform_64b_fast.toml");
        toml::from_str(toml_string).unwrap()
    }

    pub fn tuniform_64b_pfail64_psi64() -> Self {
        let toml_string = include_str!("tuniform_64b_pfail64_psi64.toml");
        toml::from_str(toml_string).unwrap()
    }

    pub fn tuniform_64b_pfail128_psi64() -> Self {
        let toml_string = include_str!("tuniform_64b_pfail128_psi64.toml");
        toml::from_str(toml_string).unwrap()
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct HpuConfig {
    pub freq: MHz,
    pub isc_depth: usize,
    pub isc_query_period: Cycle,
    pub mem_fifo_capacity: usize,
    pub mem_read_latency: usize,
    pub mem_write_latency: usize,
    pub alu_fifo_capacity: usize,
    pub alu_read_latency: usize,
    pub alu_write_latency: usize,
    pub pbs_fifo_capacity: usize,
    pub pbs_memory_capacity: usize,
    pub pbs_min_batch_size: usize,
    pub pbs_max_batch_size: usize,
    pub pbs_policy: Policy,
    pub pbs_load_unload_latency: usize,
    pub pbs_processing_latency_a: usize,
    pub pbs_processing_latency_b: usize,
    pub pbs_processing_latency_m: usize,
    pub regf_size: usize,
}

impl From<PhysicalConfig> for HpuConfig {
    fn from(phy: PhysicalConfig) -> HpuConfig {
        let pem_axi_w = phy.pc_params.pem_pc * phy.pc_params.pem_bytes_w * 8;
        let blwe_coefs = (phy.pbs_params.polynomial_size * phy.pbs_params.glwe_dimension) + 1;
        let glwe_coefs = phy.pbs_params.polynomial_size * (phy.pbs_params.glwe_dimension + 1);
        let ldst_raw_cycle = (blwe_coefs * phy.ntt_params.ct_width).div_ceil(pem_axi_w);
        let ldst_cycle = ldst_raw_cycle * 2;
        let kspbs_rd_cycle = blwe_coefs.div_ceil(phy.regf_params.coef_nb);
        let kspbs_cnst_cost = kspbs_rd_cycle;
        let rpsi = phy.ntt_params.psi * phy.ntt_params.radix;
        let ct_load_cycles = (glwe_coefs * phy.pbs_params.pbs_level).div_ceil(rpsi);
        let cmux_lat = ct_load_cycles * phy.ntt_params.batch_pbs_nb;
        let ks_cycles = cmux_lat * phy.ks_params.lbx;
        let kspbs_pbs_cost = (ks_cycles
            + phy.pbs_params.lwe_dimension * cmux_lat
            + phy.ntt_params.batch_pbs_nb * blwe_coefs.div_ceil(rpsi / 2))
            / phy.ntt_params.batch_pbs_nb;

        HpuConfig {
            freq: phy.freq,
            isc_depth: phy.isc_params.depth,
            isc_query_period: Cycle(phy.isc_params.query_period),
            mem_fifo_capacity: phy.fifo_size,
            mem_read_latency: ldst_cycle,
            mem_write_latency: ldst_cycle + 1,
            alu_fifo_capacity: phy.fifo_size,
            alu_read_latency: blwe_coefs,
            alu_write_latency: blwe_coefs + 1,
            pbs_fifo_capacity: phy.fifo_size,
            pbs_memory_capacity: phy.ntt_params.total_pbs_nb,
            pbs_min_batch_size: phy.ntt_params.min_pbs_nb,
            pbs_max_batch_size: phy.ntt_params.batch_pbs_nb,
            pbs_policy: Policy::Timeout(Cycle(phy.pbs_params.timeout)),
            pbs_load_unload_latency: kspbs_cnst_cost,
            pbs_processing_latency_a: kspbs_pbs_cost,
            pbs_processing_latency_b: kspbs_cnst_cost,
            pbs_processing_latency_m: phy.ntt_params.min_pbs_nb,
            regf_size: phy.regf_params.coef_nb
        }
    }
}
