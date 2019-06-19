use crate::hal;
use crate::workhub;

use bitcoin_hashes::{sha256d::Hash, Hash as HashTrait};
use stratum::v2::messages::{NewMiningJob, SetNewPrevhash};
use stratum::v2::V2Handler;

// TODO: move it to the stratum crate
const VERSION_MASK: u32 = 0x1fffe000;

#[derive(Copy, Clone)]
struct StratumJob {
    channel_id: u32,
    job_id: u32,
    block_height: u32,
    version: u32,
    prev_hash: Hash,
    merkle_root: Hash,
    time: u32,
    max_time: u32,
    bits: u32,
}

impl StratumJob {
    pub fn new(job_msg: NewMiningJob, prevhash_msg: SetNewPrevhash) -> Self {
        assert_eq!(job_msg.block_height, prevhash_msg.block_height);
        Self {
            channel_id: job_msg.channel_id,
            job_id: job_msg.job_id,
            block_height: job_msg.block_height,
            version: job_msg.version,
            prev_hash: Hash::from_slice(prevhash_msg.prev_hash.as_ref()).unwrap(),
            merkle_root: Hash::from_slice(job_msg.merkle_root.as_ref()).unwrap(),
            time: prevhash_msg.min_ntime,
            max_time: prevhash_msg.min_ntime + prevhash_msg.max_ntime_offset as u32,
            bits: prevhash_msg.nbits,
        }
    }
}

impl hal::BitcoinJob for StratumJob {
    fn version(&self) -> u32 {
        self.version
    }

    fn version_mask(&self) -> u32 {
        VERSION_MASK
    }

    fn previous_hash(&self) -> &Hash {
        &self.prev_hash
    }

    fn merkle_root(&self) -> &Hash {
        &self.merkle_root
    }

    fn time(&self) -> u32 {
        self.time
    }

    fn max_time(&self) -> u32 {
        self.max_time
    }

    fn bits(&self) -> u32 {
        self.bits
    }
}

struct StratumClient;

impl StratumClient {}
