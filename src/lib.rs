mod worker;
mod service;

pub use worker::Worker;
pub use service::ServiceWorker;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
