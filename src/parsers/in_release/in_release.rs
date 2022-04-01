pub enum HashType {
    None,
    MD5Sum,
    SHA1,
    SHA256,
    SHA512,
}

impl HashType {
    pub fn from_string(line: &str) -> Result<Self, ()> {
        if line.starts_with("MD5Sum") {
            return Result::Ok(Self::MD5Sum);
        } else if line.starts_with("SHA1") {
            return Result::Ok(Self::SHA1);
        } else if line.starts_with("SHA256") {
            return Result::Ok(Self::SHA256);
        } else if line.starts_with("SHA512") {
            return Result::Ok(Self::SHA512);
        }

        return Result::Err(());
    }
}

pub struct InReleaseFile {
    hash: String,
    size: usize,
    path: String,
}

impl InReleaseFile {
    pub fn new(hash: &str, size: usize, path: &str) -> Self {
        return Self {
            hash: hash.to_string(),
            size,
            path: path.to_string(),
        }
    }
}

pub struct Signature {
    version: String,
    content: String,
}

pub struct InRelease {
    pub hash: String,
    pub origin: String,
    pub label: String,
    pub suite: String,
    pub version: String,
    pub codename: String,
    pub date: String,
    pub architectures: Vec<String>,
    pub components: Vec<String>,
    pub description: String,
    pub acquire_by_hash: bool,
    pub files: Vec<(HashType, InReleaseFile)>,
    pub signature: Signature,
}
