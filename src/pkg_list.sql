create table pkg_list {
    id BIGSERIAL PRIMARY KEY,
    pkg_name UNIQUE TEXT,
    pkg_build_type SMALLINT,
    pkg_platform TEXT[],
    pkg_path TEXT[],
    }