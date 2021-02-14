mod request;
pub use request::Request;

mod response;
pub use response::Response;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
