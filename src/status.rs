pub enum Status {
    File,
    Message,
    Unknown,
}

impl Into<Status> for u8 {
    fn into(self) -> Status {
        match self {
            0 => Status::File,
            1 => Status::Message,
            _ => Status::Unknown,
        }
    }
}
