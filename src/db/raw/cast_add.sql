INSERT INTO {job}s(first_name, last_name, gender, _document)
VALUES ('{first_name}',
        '{last_name}',
        '{gender}',
        to_tsvector(coalesce('{first_name}', '') ||
                    ' ' ||
                    coalesce('{last_name}', '')));