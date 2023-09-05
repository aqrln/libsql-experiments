import Database from "libsql-experimental";

if (!process.env.SQLD_URL) {
  throw new Error("missing SQLD_URL, run `direnv allow`");
}

if (!process.env.SQLD_AUTH_TOKEN) {
  throw new Error("missing SQLD_AUTH_TOKEN, run `direnv allow`");
}

const db = new Database("replica.db", {
  syncUrl: process.env.SQLD_URL,
  // @ts-expect-error this is missing from the type definitions
  syncAuth: process.env.SQLD_AUTH_TOKEN,
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
