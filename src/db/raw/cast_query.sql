SELECT {cast}_id,
       first_name,
       last_name,
       gender,
       ts_rank_cd(_document, _query) AS _rank
FROM {cast}s,
     plainto_tsquery(CAST('english' AS REGCONFIG), CAST('{term}' AS TEXT)) _query
WHERE _document @@ _query
ORDER BY _rank DESC;