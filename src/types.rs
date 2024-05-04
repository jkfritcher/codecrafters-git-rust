use anyhow::{anyhow, Result};
use std::str;

#[derive(Debug, Clone)]
pub enum Object {
    Blob(Vec<u8>)
}

impl Object {
    pub fn blob(contents: Vec<u8>) -> Self {
        Self::Blob(contents)
    }

    pub fn deserialize(data: &[u8]) -> Result<Object> {
        // Split the header and contents into parts
        let parts = data.split(|b| *b == b'\0').collect::<Vec<&[u8]>>();
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
            "blob" => Ok(Self::Blob(contents.to_vec())),
            _ => Err(anyhow!("Unexpected object type encountered: {}", name))
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8>;
        match self {
            Object::Blob(contents) => {
                let header = format!("blob {}\0", contents.len()).into_bytes();
                data = Vec::with_capacity(header.len() + contents.len());
                data.extend_from_slice(&header);
                data.extend_from_slice(contents);
            },
        }
        data
    }
}
