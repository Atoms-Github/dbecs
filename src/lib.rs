#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]


pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub struct World {
    conn: Connection,
}
impl World{
    pub fn new() -> Self{
        let conn = Connection::open_in_memory().unwrap();
        let mut world = Self{conn};


        world.conn.execute_batch(
            r"CREATE SEQUENCE seq;
              CREATE TABLE person (
                      id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                      name            TEXT NOT NULL,
                      data            BLOB
                      );
             ").unwrap();
        world
    }
    pub fn add_1k_ents(&mut self){
        for i in 0..1_000{
            let me = Person {
                id: 0,
                name: "Steven".to_string(),
                data: None,
            };
            self.conn.execute(
                "INSERT INTO person (name, data) VALUES (?, ?)",
                params![me.name, me.data],
            ).unwrap();
        }
        // let mut prepared = self.conn.prepare().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){

        //let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
        // let person_iter = stmt.query_map([], |row| {
        //     Ok(Person {
        //         id: row.get(0).unwrap(),
        //         name: row.get(1).unwrap(),
        //         data: row.get(2).unwrap(),
        //     })
        // }).unwrap();
        //
        // for person in person_iter {
        //     println!("Found person {:?}", person.unwrap());
        // }
    }
}
use duckdb::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}
