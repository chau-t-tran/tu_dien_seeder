CREATE TABLE entries (
  id int NOT NULL AUTO_INCREMENT,
  word VARCHAR(256) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin,
  pos VARCHAR(256),
  def VARCHAR(4096),
  PRIMARY KEY (id)
);

CREATE TABLE sentences (
  id int NOT NULL AUTO_INCREMENT,
  entry_id int NOT NULL,
  eng VARCHAR(4096),
  viet VARCHAR(4096) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin,
  PRIMARY KEY (id),
  FOREIGN KEY(entry_id) REFERENCES entries(id)
);

CREATE TABLE text_audio (
  id int NOT NULL AUTO_INCREMENT,
  text VARCHAR(4096) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin UNIQUE,
  audio_url VARCHAR(1024),
  PRIMARY KEY(id)
);
