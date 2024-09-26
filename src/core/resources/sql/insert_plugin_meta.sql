INSERT INTO pluginMeta(pluginName, versions, source, tags, links)
    VALUES($1, $2, $3, $4, $5)
    RETURNING id;