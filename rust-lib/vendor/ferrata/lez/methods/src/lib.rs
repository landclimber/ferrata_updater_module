//! The guests of lez/methods as risc0-build embedded them (modules/scripts/stage-updater.sh).

pub const FERRATA_LEZ_ELF: &[u8] = include_bytes!("../guests/ferrata_lez.bin");
pub const FERRATA_LEZ_ID: [u32; 8] = [4011814273, 1227181382, 4138490684, 686456297, 461511044, 788900806, 703453085, 3741536238];
pub const FERRATA_CONSUMER_ELF: &[u8] = include_bytes!("../guests/ferrata_consumer.bin");
pub const FERRATA_CONSUMER_ID: [u32; 8] = [996172932, 3333005007, 279711388, 3118335238, 1785130952, 3429169494, 1198106347, 2521534101];
pub const SCRIPTED_ELF: &[u8] = include_bytes!("../guests/scripted.bin");
pub const SCRIPTED_ID: [u32; 8] = [4125339788, 706400589, 575763946, 1105609596, 3776471658, 2037894145, 441421769, 1632556627];
pub const FERRATA_LEZ_KECCAK_SOFT_ELF: &[u8] = include_bytes!("../guests/ferrata_lez_keccak_soft.bin");
pub const FERRATA_LEZ_KECCAK_SOFT_ID: [u32; 8] = [2361780719, 3568045738, 1568749512, 3980157, 2025066800, 2237385977, 3062079740, 1053533680];
pub const FERRATA_CONSUMER_KECCAK_ACCEL_ELF: &[u8] = include_bytes!("../guests/ferrata_consumer_keccak_accel.bin");
pub const FERRATA_CONSUMER_KECCAK_ACCEL_ID: [u32; 8] = [2431622138, 3373995337, 3895325486, 299634510, 293553235, 2332756315, 204674148, 3426996969];
pub const FERRATA_BRIDGE_ELF: &[u8] = include_bytes!("../guests/ferrata_bridge.bin");
pub const FERRATA_BRIDGE_ID: [u32; 8] = [910391940, 2636513986, 4131790780, 2257667710, 2241943778, 3469975171, 4183704161, 360030620];
pub const FERRATA_HOLDINGS_ELF: &[u8] = include_bytes!("../guests/ferrata_holdings.bin");
pub const FERRATA_HOLDINGS_ID: [u32; 8] = [1547673255, 1894946372, 1526052474, 16453272, 4121592029, 1512027490, 3913892688, 2438298793];
