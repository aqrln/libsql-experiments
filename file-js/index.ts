import Database from "libsql-experimental";

const db = new Database("dev.db");

db.exec(`
  CREATE TABLE IF NOT EXISTS user (
    id    INTEGER PRIMARY KEY,
    email TEXT
  );
`);

db.exec(`
  INSERT INTO user (email)
  VALUES ('user@example.com');
`);

const result = db.prepare(`SELECT * FROM user`).all();
console.log(result);
