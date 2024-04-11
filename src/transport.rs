struct RPC {
    From : String,
    Payload : Vec<u8>,
}

pub trait Transport<T> {
    fn consume();
    fn connect(Trans)
}