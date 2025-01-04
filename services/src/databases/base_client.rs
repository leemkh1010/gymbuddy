pub trait Singleton<T> {
    fn get_instance(&self) -> &T
    where
        Self: Sized;
}

pub trait ExternalClient {
    fn new(host: &str, port: u16, db: &str) -> Self
    where
        Self: Sized;
    // async fn connect<T, E>(&mut self) -> Result<T, E>;
    // fn disconnect(&self) -> Result<(), ()>;
    // fn is_connected(&self) -> bool;
}
