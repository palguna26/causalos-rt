use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tracing::info;

pub struct CausalEventHeader {
    pub timestamp: u64,
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

    pub fn append_event(&mut self, event_type: &str, data: &[u8]) -> anyhow::Result<()> {
        let type_bytes = event_type.as_bytes();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        // Binary format: [Timestamp (8)] [Type Len (4)] [Type] [Data Len (4)] [Data]
        self.writer.write_all(&timestamp.to_le_bytes())?;
        self.writer.write_all(&(type_bytes.len() as u32).to_le_bytes())?;
        self.writer.write_all(type_bytes)?;
        self.writer.write_all(&(data.len() as u32).to_le_bytes())?;
        self.writer.write_all(data)?;
        self.writer.flush()?;
        
        info!("Appended event: {} ({} bytes)", event_type, data.len());
        Ok(())
    }

    pub fn read_last_n(&self, n: usize) -> anyhow::Result<Vec<(CausalEventHeader, Vec<u8>)>> {
        let mut file = File::open(&self.path)?;
        let mut events = Vec::new();
        
        // Simple linear scan for Alpha
        loop {
            let mut ts_buf = [0u8; 8];
            if file.read_exact(&mut ts_buf).is_err() { break; }
            let timestamp = u64::from_le_bytes(ts_buf);

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

            events.push((CausalEventHeader { timestamp, event_type: type_bytes }, data));
        }

        Ok(events.into_iter().rev().take(n).collect())
    }
}
