CREATE TABLE dags (
                      id UUID PRIMARY KEY,
                      name TEXT NOT NULL
);

CREATE TABLE nodes (
                       id UUID PRIMARY KEY,
                       dag_id UUID REFERENCES dags(id),
                       label TEXT NOT NULL
);

CREATE TABLE edges (
                       id UUID PRIMARY KEY,
                       source UUID REFERENCES nodes(id),
                       target UUID REFERENCES nodes(id),
                       dag_id UUID REFERENCES dags(id)
);
