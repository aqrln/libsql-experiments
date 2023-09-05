import Database from "libsql-experimental";

if (!process.env.SQLD_URL) {
  throw new Error("missing SQLD_URL, run `direnv allow`");
}

if (!process.env.SQLD_AUTH_TOKEN) {
  throw new Error("missing SQLD_AUTH_TOKEN, run `direnv allow`");
}

const db = new Database(process.env.SQLD_URL, {
  // @ts-expect-error this is missing from the type definitions
  authToken: process.env.SQLD_AUTH_TOKEN,
});

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
