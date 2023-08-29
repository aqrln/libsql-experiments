import Database from "libsql-experimental";

if (!process.env.SQLD_URL) {
  throw new Error("missing SQLD_URL, run `direnv allow`");
}

const db = new Database("replica.db", {
  syncUrl: process.env.SQLD_URL,
});

db.sync();

db.exec(`
  CREATE TABLE IF NOT EXISTS user (
    id    INTEGER PRIMARY KEY,
    email TEXT
  );
`);

const beforeInsert = db.prepare(`SELECT * FROM user`).all();
console.log("before insert:", beforeInsert);

db.prepare(`
  INSERT INTO user (email)
  VALUES (?);
`).run(`${Math.random()}@example.com`);

const afterInsert = db.prepare(`SELECT * FROM user`).all();
console.log("after insert:", afterInsert);
