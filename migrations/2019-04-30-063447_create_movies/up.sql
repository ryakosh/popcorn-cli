CREATE TABLE IF NOT EXISTS movies (
  movie_id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL DEFAULT 'Description is not provided.',
  poster VARCHAR(19) NOT NULL,
  genres TEXT[] NOT NULL,
  languages TEXT[] NOT NULL,
  release_country VARCHAR(2) NOT NULL,
  release_date DATE NOT NULL,
  duration SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS movies_writers (
  movie_id INTEGER,
  writer_id INTEGER,
  PRIMARY KEY(movie_id, writer_id)
);

CREATE TABLE IF NOT EXISTS movies_directors (
  movie_id INTEGER,
  director_id INTEGER,
  PRIMARY KEY(movie_id, director_id)
);

CREATE TABLE IF NOT EXISTS movies_artists (
  movie_id INTEGER,
  artist_id INTEGER,
  PRIMARY KEY(movie_id, artist_id)
);

CREATE INDEX IF NOT EXISTS idx_genres ON movies USING GIN(genres);
CREATE INDEX IF NOT EXISTS idx_languages ON movies USING GIN(languages);