CREATE TABLE entries (
  id int NOT NULL,
  word varchar(255),
  pos varchar(255),
  def varchar(2040),
  PRIMARY KEY (id)
);

CREATE TABLE sentences (
  id int NOT NULL,
  entry_id int NOT NULL,
  eng varchar(1020),
  viet varchar(1020),
  PRIMARY KEY (id),
  FOREIGN KEY(entry_id) REFERENCES entries(id)
);
