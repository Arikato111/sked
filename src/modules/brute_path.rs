use crate::utils::WordlistType;

/// ## Accepted http status codes.
pub enum AcceptStatus {
    /// ## Accept all status codes.
    All,
    /// ## Accept specific status codes.
    Specific(Vec<u16>),
}

/// ## Brute force website's path url.
pub struct BrutePath {
    /// ## Target url.
    url: String,
    /// Wordlist type. (Range, File)
    wordlist: WordlistType,
    /// Accepted http status codes.
    accept_status: AcceptStatus,
    /// Download found files.
    download: bool,
}

impl BrutePath {
    pub fn new(
        url: String,
        wordlist: WordlistType,
        accept_status: AcceptStatus,
        download: bool,
    ) -> Self {
        Self {
            url,
            wordlist,
            accept_status,
            download,
        }
    }
    pub async fn run(&self) {
        let client = reqwest::Client::new();
    }
}
