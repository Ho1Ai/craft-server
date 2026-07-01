create table pkg_list_actual (
    id BIGSERIAL PRIMARY KEY,
    pkg_name TEXT UNIQUE NOT NULL,
    pkg_version TEXT NOT NULL,
    pkg_build_type SMALLINT DEFAULT 0,
    pkg_platform TEXT[],
    platform_sensitive BOOLEAN,
    pkg_dependencies_list TEXT[] DEFAULT '{}',
    pkg_cdn_path TEXT[] DEFAULT '{}',
    pkg_hash BYTEA,
    pkg_size BIGINT,
    pkg_last_update TIMESTAMP DEFAULT NOW(),
    pkg_posted TIMESTAMP,
    );

/*
    peers list table will be remade
*/

create table pkg_peers_list(
    id BIGSERIAL PRIMARY KEY,
    pkg_name TEXT REFERENCES pkg_list(pkg_name),
    peers_list TEXT[],
)