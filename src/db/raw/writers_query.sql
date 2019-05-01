SELECT writer_id,
       first_name,
       last_name,
       gender,
       ts_rank_cd(_document, _query) AS _rank
FROM writers,
     plainto_tsquery(CAST('english' AS REGCONFIG), CAST('{}' AS TEXT)) _query
WHERE _document @@ _query
ORDER BY _rank DESC;