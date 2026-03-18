CREATE TABLE IF NOT EXISTS repos (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS constructs (
    id TEXT PRIMARY KEY,
    repo_id TEXT NOT NULL REFERENCES repos(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    topology TEXT NOT NULL DEFAULT 'circular' CHECK (topology IN ('circular', 'linear')),
    tags TEXT NOT NULL DEFAULT '[]',
    sequence TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_constructs_repo ON constructs(repo_id);

CREATE TABLE IF NOT EXISTS construct_versions (
    id TEXT PRIMARY KEY,
    construct_id TEXT NOT NULL REFERENCES constructs(id) ON DELETE CASCADE,
    version_num INTEGER NOT NULL,
    sequence TEXT NOT NULL,
    parts_json TEXT NOT NULL DEFAULT '[]',
    snapshot_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_versions_construct ON construct_versions(construct_id);

CREATE TABLE IF NOT EXISTS parts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    part_type TEXT NOT NULL CHECK (part_type IN (
        'promoter', 'cds', 'terminator', 'ori', 'marker',
        'tag', 'linker', 'signal_peptide', 'regulatory', 'other'
    )),
    sequence TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_parts_type ON parts(part_type);
CREATE INDEX IF NOT EXISTS idx_parts_name ON parts(name);

CREATE TABLE IF NOT EXISTS construct_parts (
    id TEXT PRIMARY KEY,
    construct_id TEXT NOT NULL REFERENCES constructs(id) ON DELETE CASCADE,
    part_id TEXT NOT NULL REFERENCES parts(id),
    position INTEGER NOT NULL DEFAULT 0,
    strand INTEGER NOT NULL DEFAULT 1 CHECK (strand IN (1, -1)),
    order_index INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_cparts_construct ON construct_parts(construct_id);

CREATE TABLE IF NOT EXISTS simulator_runs (
    id TEXT PRIMARY KEY,
    construct_id TEXT NOT NULL REFERENCES constructs(id) ON DELETE CASCADE,
    host TEXT NOT NULL DEFAULT '',
    params TEXT NOT NULL DEFAULT '{}',
    results TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS chat_messages (
    id TEXT PRIMARY KEY,
    construct_id TEXT,
    role TEXT NOT NULL CHECK (role IN ('system', 'user', 'assistant')),
    content TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_chat_construct ON chat_messages(construct_id);

CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS notebooks (
    id TEXT PRIMARY KEY,
    repo_id TEXT NOT NULL REFERENCES repos(id) ON DELETE CASCADE,
    title TEXT NOT NULL DEFAULT 'Untitled Notebook',
    content TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_notebooks_repo ON notebooks(repo_id);
