use utils;

// json!に必要
#[derive(Serialize, Deserialize, Debug)]
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
        
        let conn = utils::get_connection();
        
        let result = conn.execute(
            "INSERT INTO Memo (content) VALUES (?1)",
            &[&self.content]
        );

        result.is_ok()
    }

    pub fn all() -> Vec<Memo> {
        let conn = utils::get_connection();
        let mut stmt = conn.prepare("SELECT id, content FROM Memo").unwrap();
        let mut memo_iter = stmt.query(&[]).unwrap();

        let mut memo_vec: Vec<Memo> = Vec::new();
        while let Some(Ok(memo)) = memo_iter.next() {
            memo_vec.push(
                Memo {
                    id: memo.get(0),
                    content: memo.get(1),
                }
            );
        }

        memo_vec
    }
    
}
