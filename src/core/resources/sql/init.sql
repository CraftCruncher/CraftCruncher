CREATE TABLE IF NOT EXISTS pluginMeta(
    id SERIAL PRIMARY KEY,
    pluginName TEXT,
    artifactName TEXT,
    versions JSONB[],
    source TEXT,
    tags TEXT[],
    links TEXT[]
);

CREATE INDEX IF NOT EXISTS pluginNamesAlphabetical
    ON pluginMeta (artifactName);

CREATE TABLE IF NOT EXISTS authors(
    id SERIAL PRIMARY KEY,
    authorName TEXT,
    creationDate TIMESTAMP,
    pluginPortal TEXT
);

CREATE TABLE IF NOT EXISTS authorPluginRelation(
    authorId INT,
    pluginId INT,
    CONSTRAINT fk_author FOREIGN KEY(authorId) REFERENCES authors(id),
    CONSTRAINT fk_plugin FOREIGN KEY(pluginId) REFERENCES pluginMeta(id)
);