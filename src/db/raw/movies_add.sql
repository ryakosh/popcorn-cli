INSERT INTO movies(title, description, poster, genres,
                   languages, release_country, release_date,
                   duration)
VALUES ('{title}', '{description}', '{poster}', '{{{genres}}}',
        '{{{languages}}}', '{release_country}', '{release_date}',
        {duration})
RETURNING movie_id;