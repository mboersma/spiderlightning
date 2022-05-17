use std::{
    fs::{File, self},
    io::{Read, Write},
    path::PathBuf,
};

pub use kv::add_to_linker;
use kv::*;

wit_bindgen_wasmtime::export!("../../wit/kv.wit");

#[derive(Default)]
pub struct KvFilesystem {
    path: String,
}

impl KvFilesystem {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl kv::Kv for KvFilesystem {
    type ResourceDescriptor = u64;

    fn get_kv(&mut self) -> Result<Self::ResourceDescriptor, Error> {
        Ok(0)
    }

    /// Output the value of a set key.
    /// If key has not been set, return empty.
    fn get(&mut self, rd: &Self::ResourceDescriptor, key: &str) -> Result<PayloadResult, Error> {
        if *rd != 0 {
            return Err(Error::Error);
        }

        let mut file = match File::open(path(key, &self.path)?) {
            Ok(f) => f,
            Err(_) => return Ok(Vec::new()),
        };

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Create a key-value pair.
    fn set(
        &mut self,
        rd: &Self::ResourceDescriptor,
        key: &str,
        value: PayloadParam<'_>,
    ) -> Result<(), Error> {
        if *rd != 0 {
            return Err(Error::Error);
        }
        let mut file = File::create(path(key, &self.path)?)?;
        file.write_all(value)?;
        Ok(())
    }

    /// Delete a key-value pair.
    fn delete(
        &mut self,
        rd: &Self::ResourceDescriptor,
        key: &str
    ) -> Result<(), Error> {
        if *rd != 0 {
            return Err(Error::Error);
        }
        fs::remove_file(path(key, &self.path)?)?;
        Ok(())
    }
}

/// Return the absolute path for the file corresponding to the given key.
fn path(name: &str, base: &str) -> Result<PathBuf, anyhow::Error> {
    Ok(PathBuf::from(base).join(name))
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Error
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::Error
    }
}
