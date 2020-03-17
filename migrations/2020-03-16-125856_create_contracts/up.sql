CREATE TABLE contracts (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  vendor VARCHAR NOT NULL,
  starts_on TEXT NOT NULL,
  ends_on TEXT NOT NULL,
  price NUMERIC NOT NULL
)
