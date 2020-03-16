CREATE TABLE contracts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  vendor VARCHAR NOT NULL,
  starts_on TEXT NOT NULL,
  ends_on TEXT NOT NULL,
  price REAL NOT NULL
)
