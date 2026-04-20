use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Clone)]
pub struct CausalEventHeader {
    pub timestamp: u64,
    pub parent_hash: u64,
    pub event_type: Vec<u8>,
}

pub struct CausalLedger {
    path: PathBuf,
    writer: BufWriter<File>,
}

impl CausalLedger {
    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path_buf)?;
        
        Ok(Self {
            path: path_buf,
            writer: BufWriter::new(file),
        })
    }

    pub fn append_event(&mut self, event_type: &str, data: &[u8]) -> anyhow::Result<u64> {
        self.append_with_parent(event_type, data, 0)
    }

    pub fn append_with_parent(&mut self, event_type: &str, data: &[u8], parent_hash: u64) -> anyhow::Result<u64> {
        let type_bytes = event_type.as_bytes();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        // Binary format: [Timestamp (8)] [Parent Hash (8)] [Type Len (4)] [Type] [Data Len (4)] [Data]
        self.writer.write_all(&timestamp.to_le_bytes())?;
        self.writer.write_all(&parent_hash.to_le_bytes())?;
        self.writer.write_all(&(type_bytes.len() as u32).to_le_bytes())?;
        self.writer.write_all(type_bytes)?;
        self.writer.write_all(&(data.len() as u32).to_le_bytes())?;
        self.writer.write_all(data)?;
        self.writer.flush()?;
        
        let event_hash = fxhash::hash64(data); // In v2, the hash of the payload is the event ID
        info!("Appended event: {} (hash: {:x}, parent: {:x})", event_type, event_hash, parent_hash);
        Ok(event_hash)
    }

    pub fn read_last_n(&self, n: usize) -> anyhow::Result<Vec<(CausalEventHeader, Vec<u8>)>> {
        if !self.path.exists() { return Ok(vec![]); }
        let mut file = File::open(&self.path)?;
        let mut events = Vec::new();
        
        loop {
            let mut ts_buf = [0u8; 8];
            if file.read_exact(&mut ts_buf).is_err() { break; }
            let timestamp = u64::from_le_bytes(ts_buf);

            let mut parent_buf = [0u8; 8];
            file.read_exact(&mut parent_buf)?;
            let parent_hash = u64::from_le_bytes(parent_buf);

            let mut type_len_buf = [0u8; 4];
            file.read_exact(&mut type_len_buf)?;
            let type_len = u32::from_le_bytes(type_len_buf) as usize;

            let mut type_bytes = vec![0u8; type_len];
            file.read_exact(&mut type_bytes)?;

            let mut data_len_buf = [0u8; 4];
            file.read_exact(&mut data_len_buf)?;
            let data_len = u32::from_le_bytes(data_len_buf) as usize;

            let mut data = vec![0u8; data_len];
            file.read_exact(&mut data)?;

            events.push((CausalEventHeader { timestamp, parent_hash, event_type: type_bytes }, data));
        }

        Ok(events.into_iter().rev().take(n).collect())
    }
}

