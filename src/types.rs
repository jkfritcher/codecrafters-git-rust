use anyhow::{anyhow, Result};
use std::str;

use crate::util::split_header;

#[derive(Debug, Clone)]
pub enum Object {
    Blob(Vec<u8>),
    Tree(Vec<TreeEntry>),
}

impl Object {
    pub fn name(&self) -> String {
        match self {
            Self::Blob(_) => "blob".to_string(),
            Self::Tree(_) => "tree".to_string(),
        }
    }

    pub fn deserialize(data: &[u8]) -> Result<Object> {
        // Split the header and contents into parts
        let parts = split_header(data)?;
        if parts.len() != 2 {
            return Err(anyhow!("data did not split into 2 parts. Found {} parts instead.", parts.len()));
        }
        let contents = parts[1];

        // Split the header into the type and content length parts
        let header = parts[0].split(|b| *b == b' ').collect::<Vec<&[u8]>>();
        if header.len() != 2 {
            return Err(anyhow!("header did not split into 2 parts. Found {} parts instead.", header.len()));
        }
        let name = str::from_utf8(header[0])?;
        let encoded_len = str::from_utf8(header[1])?.parse::<usize>()?;
        if contents.len() != encoded_len {
            return Err(anyhow!("The encoded length and actual length of the contents do not match, '{}' vs '{}'", contents.len(), encoded_len));
        }
        // Match the type and build the appropriate object
        match name {
            "blob" => Ok(Self::blob(contents.to_vec())?),
            "tree" => {
                let mut entries = Vec::new();
                let mut start: usize = 0;
                let mut i: usize;
                loop {
                    i = 0;
                    while start + i < contents.len() {
                        if contents[start + i] != b'\0' {
                            i += 1;
                            continue;
                        }
                        i += 21;
                        break;
                    }
                    entries.push(TreeEntry::deserialize(&contents[start..start+i])?);
                    start += i;
                    if start >= contents.len() {
                        break;
                    }
                }

                Ok(Self::tree(entries)?)
            },
            _ => Err(anyhow!("Unexpected object type encountered: {}", name))
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8>;
        match self {
            Self::Blob(contents) => {
                let header = format!("blob {}\0", contents.len()).into_bytes();
                data = Vec::with_capacity(header.len() + contents.len());
                data.extend_from_slice(&header);
                data.extend_from_slice(contents);
            },
            Self::Tree(entries) => {
                let mut contents: Vec<u8> = Vec::with_capacity(64 * entries.len()); // initial 36 bytes for file name
                for entry in entries {
                    contents.extend_from_slice(&entry.serialize());
                }
                data = Vec::with_capacity(contents.len() + 11); // 11 for assumed header length
                data.extend_from_slice(format!("tree {}\0", contents.len()).as_bytes());
                data.extend_from_slice(&contents);
            },
        }
        data
    }

    pub fn blob(contents: Vec<u8>) -> Result<Self> {
        Ok(Self::Blob(contents))
    }

    pub fn tree(entries: Vec<TreeEntry>) -> Result<Self> {
        Ok(Self::Tree(entries))
    }
}

#[derive(Debug, Clone)]
pub struct TreeEntry {
    mode: String,
    name: String,
    hash: Vec<u8>,
}

impl TreeEntry {
    pub fn new(mode: String, name: String, hash: &[u8]) -> Result<Self> {
        let hash_len = hash.len();
        if hash_len != 20 {
            return Err(anyhow!("Hash is an invalid length, {}", hash_len))
        }
        let mut our_hash = Vec::with_capacity(hash_len);
        our_hash.extend_from_slice(&hash);
        Ok(Self {
            mode: mode,
            name: name,
            hash: our_hash,
        })
    }

    pub fn mode(&self) -> &str {
        return self.mode.as_ref()
    }

    pub fn name(&self) -> &str {
        return self.name.as_ref()
    }

    pub fn hash(&self) -> &[u8] {
        return self.hash.as_ref()
    }

    pub fn deserialize(contents: &[u8]) -> Result<Self> {
        let len = contents.len();
        let hash = &contents[len-20..];
        let mut i: usize = 0;
        while contents[i] != b' ' {
            i += 1;
        }
        let mode = String::from_utf8_lossy(&contents[0..i]).to_string();
        let name = String::from_utf8_lossy(&contents[i+1..len-21]).to_string();
        let mut new_hash = vec!(0u8; 20);
        new_hash.copy_from_slice(&hash);

        Ok(Self::new(mode, name, &new_hash)?)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let entry_len = self.mode.as_bytes().len() + self.name.as_bytes().len() + 20 + 2;
        let mut data = Vec::with_capacity(entry_len);
        data.extend_from_slice(format!("{} {}\0", self.mode, self.name).as_bytes());
        data.extend_from_slice(&self.hash);
        data
    }
}
