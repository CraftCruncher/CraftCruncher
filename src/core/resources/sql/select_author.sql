SELECT id FROM authors
    WHERE authorName = $1 AND creationDate = $2 AND pluginPortal = $3;