pub struct InReleaseFile {
    hash: String,
    size: usize,
    path: String,
}

pub struct Signature {
    version: String,
    content: String,
}

pub struct InRelease {
    hash: String,
    origin: String,
    suite: String,
    version: String,
    codename: String,
    date: String,
    architectures: Vec<String>,
    components: Vec<String>,
    description: String,
    acquire_by_hash: bool,
    files: Vec<(String, InReleaseFile)>,
    signature: Signature,
}

enum State {
    NotStarted,
    Header,
    Hashes,
    PostHashes,
    Signature,
}




pub fn parse_in_release(content: &str) -> Result<InRelease, ()> {
    let mut current_state = State::NotStarted;

    for line in content.lines() {
        if line.contains("BEGIN PGP SIGNED MESSAGE") {
            current_state = State::Header;
        }
    }
    return Result::Err(());
}