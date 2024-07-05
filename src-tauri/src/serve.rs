
use std::sync::atomic::{AtomicI32, Ordering};


static ID: AtomicI32 = AtomicI32::new(1);

struct BookMark {
    id: String,
    name: String,
    page: i32,
    description: String,
    created_at: String,
    updated_at: String,
}

impl BookMark {
    fn new(name: String, description: String) -> Self {
        let created_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let id = ID.fetch_add(1, Ordering::SeqCst);
        BookMark {
            name,
            page: i32::default(),
            description,
            created_at,
            updated_at: String::default(),
            id: format!("{:03}", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_new() {
        let bm = BookMark::new("test".to_string(), "test".to_string());
        assert_eq!(bm.id, "001");
        assert_eq!(bm.name, "test");
        assert_eq!(bm.description, "test");
        assert_eq!(bm.updated_at, "");
    }
}