use sqlite;

pub fn tester() {
    let connection = sqlite::open("./db.sqlite3").unwrap();

    let query = "
            CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Bob', 69);
        ";
    connection.execute(query).unwrap();
}
