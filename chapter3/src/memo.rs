use std::vec::Vec;
use rusqlite::Connection;

pub fn get_connection() -> Connection {
    let db_path = &"data.db";
    Connection::open(db_path).unwrap()
}

// json!に必要
#[derive(Serialize, Deserialize)]
pub struct Memo {
    pub id: u32,
    pub content: String,
}

impl Memo {
    
    pub fn new(content: String) -> Result<Memo, String> {
        let memo = Memo {
            id: 1,
            content: content,
        };
        memo.insert();
        Ok(memo)
    }
    
    pub fn insert(&self) -> bool {
        
        let conn = get_connection();
        
        let result = conn.execute(
            "INSERT INTO Memo (content) VALUES (?1)",
            &[&self.content]
        );
        
        match result {
            Ok(_) => { true },
            Err(_) => { false }
        }
    }

    pub fn all() -> Vec<Memo> {
        
        let conn = get_connection();
        
        let mut stmt = conn.prepare("SELECT id, content FROM Memo").unwrap();
        let memo_iter = stmt.query_map(&[], |row| {
            Memo {
                id: row.get(0),
                content: row.get(1),
            }
        }).unwrap();
        
        let mut memo_vec: Vec<Memo> = Vec::new();
        for memo in memo_iter {
            memo_vec.push(memo.unwrap());
        }

        memo_vec
    }

    pub fn get(id: &str)-> Memo{
        let conn = get_connection();
        let mut stmt = conn.prepare("SELECT id, content FROM Memo WHERE id=(?1)").unwrap();

        let mut memo_iter = stmt.query_map(&[&id], |row| {
            Memo {
                id: row.get(0),
                content: row.get(1),
            }
        }).unwrap();

        memo_iter.nth(0).unwrap().unwrap()
    }
    
    pub fn update(id: &str, content: &str)-> bool{
        let conn = get_connection();
        let result = conn.execute(
            "UPDATE Memo SET content=(?1) WHERE id=(?2)",
            &[&content, &id]
        );

        result.is_ok()
    }

    pub fn delete(id: &str)-> bool{
        let conn = get_connection();
        let result = conn.execute(
            "DELETE FROM Memo WHERE id=(?1)",
            &[&id]
        );

        result.is_ok()
    }
}
