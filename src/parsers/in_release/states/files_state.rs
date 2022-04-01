use crate::parsers::in_release::HashType;
use crate::parsers::in_release::in_release_builder::InReleaseBuilder;
use crate::parsers::in_release::in_release_states::{ParserState, State};
use crate::parsers::InReleaseFile;

pub struct FilesState {
    current_hash: HashType,
    md5_sum_files: Vec<InReleaseFile>,
    sha1_files: Vec<InReleaseFile>,
    sha256_files: Vec<InReleaseFile>,
    sha512_files: Vec<InReleaseFile>,
    acquire_by_hash: Option<bool>,
}

impl FilesState {
    pub fn new() -> Self {
        return FilesState {
            current_hash: HashType::None,
            md5_sum_files: Vec::new(),
            sha1_files: Vec::new(),
            sha256_files: Vec::new(),
            sha512_files: Vec::new(),
            acquire_by_hash: Option::None,
        };
    }

    fn parse_file_line(line: &str) -> Result<InReleaseFile, ()> {
        let mut iter = line.trim().split_whitespace();

        // TODO: Error enums.
        let hash = iter.next()
            .ok_or(())?;
        let size: usize = iter.next()
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;
        let path = iter.next()
            .ok_or(())?;

        return Result::Ok(InReleaseFile::new(hash, size, path));
    }
}

impl ParserState for FilesState {
    fn parse_line(&mut self, line: &str) -> Result<(), ()> {
        if line.ends_with(':') {
            self.current_hash = HashType::from_string(line)?
        }

        if line.starts_with("Acquire-By-Hash") {
            match line.split_whitespace().nth(1).ok_or(())? {
                "yes" => self.acquire_by_hash = Option::Some(true),
                "no" => self.acquire_by_hash = Option::Some(false),
                &_ => { return Result::Err(()); }
            }
        }

        let file = Self::parse_file_line(line)?;

        match self.current_hash {
            HashType::None => {
                return Result::Err(());
            }
            HashType::MD5Sum => {
                self.md5_sum_files.push(file);
            }
            HashType::SHA1 => {
                self.sha1_files.push(file);
            }
            HashType::SHA256 => {
                self.sha256_files.push(file);
            }
            HashType::SHA512 => {
                self.sha512_files.push(file);
            }
        }

        return Result::Ok(());
    }

    fn is_my_state(&self, line: &str) -> bool {
        if HashType::from_string(line).is_ok() {
            return true;
        }

        return false;
    }

    fn get_state(&self) -> State {
        return State::Files;
    }

    fn dump_data(&mut self, target: &mut InReleaseBuilder) {
        target.md5_sum_files.append(&mut self.md5_sum_files);
        target.sha1_files.append(&mut self.sha1_files);
        target.sha256_files.append(&mut self.sha256_files);
        target.sha512_files.append(&mut self.sha512_files);
        target.acquire_by_hash = self.acquire_by_hash;

        return;
    }
}
