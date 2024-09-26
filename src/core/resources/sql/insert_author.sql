INSERT INTO authors(authorName, creationDate, pluginPortal)
    VALUES($1, $2, $3)
    RETURNING id;