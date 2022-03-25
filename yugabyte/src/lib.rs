pub mod db_connection;
pub mod engine;
pub mod model;
pub mod schema;
pub mod util;
pub mod context;


#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
