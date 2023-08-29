import Database from "libsql-experimental";

if (!process.env.SQLD_URL) {
  throw new Error("missing SQLD_URL, run `direnv allow`");
}

const db = new Database(process.env.SQLD_URL);

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
