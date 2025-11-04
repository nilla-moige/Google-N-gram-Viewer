/// A request from the client to the server
#[derive(Debug, PartialEq)]
pub enum Request {
    /// Add the document `doc` to the archive
    Publish { doc: String },
    /// Search for the word `word` in the archive
    Search { word: String },
    /// Retrieve the document with the index `id` from the archive
    Retrieve { id: usize },
}
impl Request {
    // TODO:
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = match self {
            Request::Publish { doc } => {
                let mut result = vec![1u8];
                let doc_bytes = doc.as_bytes();
                let len = doc_bytes.len() as u32;
                result.extend_from_slice(&len.to_be_bytes());
                result.extend_from_slice(doc_bytes);
                result
            }
            Request::Search { word } => {
                let mut result = vec![2u8];
                let word_bytes = word.as_bytes();
                let len = word_bytes.len() as u32;
                result.extend_from_slice(&len.to_be_bytes());
                result.extend_from_slice(word_bytes);
                result
            }
            Request::Retrieve { id } => {
                let mut result = vec![3u8];
                result.extend_from_slice(&id.to_be_bytes());
                result
            }
        };
        bytes
    }
    // TODO:
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut tag_buf = [0u8; 1];
        reader.read_exact(&mut tag_buf).ok()?;
        match tag_buf[0] {
            1 => {
                let mut len_buf = [0; 4];
                reader.read_exact(&mut len_buf).unwrap();
                let len = u32::from_be_bytes(len_buf) as usize;
                let mut doc_buf = vec![0; len];
                reader.read_exact(&mut doc_buf).unwrap();
                let doc = String::from_utf8(doc_buf).ok()?;
                Some(Request::Publish { doc })
            }
            2 => {
                let mut len_buf = [0; 4];
                reader.read_exact(&mut len_buf).unwrap();
                let len = u32::from_be_bytes(len_buf) as usize;
                let mut word_buf = vec![0; len];
                reader.read_exact(&mut word_buf).unwrap();
                let word = String::from_utf8(word_buf).ok()?;
                Some(Request::Search { word })
            }
            3 => {
                let mut id_buf = [0u8; 8];
                reader.read_exact(&mut id_buf).ok()?;
                let id = usize::from_be_bytes(id_buf);
                Some(Request::Retrieve { id })
            }

            _ => None,
        }
    }
}

/// A response from the server to the client
#[derive(Debug, PartialEq)]
pub enum Response {
    /// The document was successfully added to the archive with the given index
    PublishSuccess(usize),
    /// The search for the word was successful, and the indices of the documents containing the
    /// word are returned
    SearchSuccess(Vec<usize>),
    /// The retrieval of the document was successful, and the document is returned
    RetrieveSuccess(String),
    /// The request failed
    Failure,
}
impl Response {
    // TODO:
    // Convert the request `self` into a byte vector. See the assignment handout for suggestions on
    // how to represent the request as a series of bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = match self {
            Response::PublishSuccess(id) => {
                let mut result = vec![1u8];
                result.extend_from_slice(&id.to_be_bytes());
                result
            }
            Response::SearchSuccess(ids) => {
                let mut result = vec![2u8];
                let len = ids.len() as u32;
                result.extend_from_slice(&len.to_be_bytes());
                for id in ids {
                    result.extend_from_slice(&id.to_be_bytes());
                }
                result
            }
            Response::RetrieveSuccess(doc) => {
                let mut result = vec![3u8];
                let doc_bytes = doc.as_bytes();
                let len = doc_bytes.len() as u32;
                result.extend_from_slice(&len.to_be_bytes());
                result.extend_from_slice(doc_bytes);
                result
            }
            Response::Failure => vec![4u8],
        };
        bytes
    }
    // TODO:
    // Read a request from `reader` and return it. Calling `to_bytes` from above and then calling
    // `from_bytes` should return the original request. If the request is invalid, return `None`.
    pub fn from_bytes<R: std::io::Read>(mut reader: R) -> Option<Self> {
        let mut tag_buf = [0u8; 1];
        reader.read_exact(&mut tag_buf).ok()?;
        match tag_buf[0] {
            1 => {
                let mut id_buf = [0u8; 8];
                reader.read_exact(&mut id_buf).ok()?;
                let id = usize::from_be_bytes(id_buf);
                Some(Response::PublishSuccess(id))
            }
            
            2 => {
                let mut len_buf = [0; 4];
                reader.read_exact(&mut len_buf).unwrap();
                let len = u32::from_be_bytes(len_buf) as usize;
                let mut ids = Vec::with_capacity(len);
                for _ in 0..len {
                    let mut id_buf = [0; 8];
                    reader.read_exact(&mut id_buf).unwrap();
                    let id = usize::from_be_bytes(id_buf);
                    ids.push(id);
                }
                Some(Response::SearchSuccess(ids))
            }
            3 => {
                let mut len_buf = [0; 4];
                reader.read_exact(&mut len_buf).unwrap();
                let len = u32::from_be_bytes(len_buf) as usize;
                let mut doc_buf = vec![0; len];
                reader.read_exact(&mut doc_buf).unwrap();
                let doc = String::from_utf8(doc_buf).ok()?;
                Some(Response::RetrieveSuccess(doc))
            }
            4 => Some(Response::Failure),
            _ => None,
        }
    }
}
