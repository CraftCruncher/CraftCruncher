CREATE TABLE IF NOT EXISTS pluginMeta(
    id BINARY(16) PRIMARY KEY,
    pluginName TEXT,
    artifactName TEXT,
    versions JSONB[],
    authors BINARY(16)[],
    source TEXT,
    tags TEXT[],
    links TEXT[]
);

CREATE INDEX IF NOT EXISTS pluginNamesAlphabetical
    ON pluginMeta (artifactName);

CREATE TABLE IF NOT EXISTS authors(
    id BINARY(16) PRIMARY KEY,
    plugins BINARY(16)[],
    authorName TEXT,
    createdDate DATE,
    pluginPortals TEXT[]
);