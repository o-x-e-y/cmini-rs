-- corpus metadata with total counts for conversion
CREATE TABLE IF NOT EXISTS corpus (
    corpus TEXT NOT NULL PRIMARY KEY,
    char_total INTEGER NOT NULL,
    bigram_total INTEGER NOT NULL,
    skipgram_total INTEGER NOT NULL,
    trigram_total INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS chars (
    corpus TEXT NOT NULL,
    ngram TEXT NOT NULL CHECK(length(ngram) = 1),
    frequency REAL NOT NULL,
    PRIMARY KEY (corpus, ngram),
    FOREIGN KEY (corpus) REFERENCES corpus(corpus)
);

CREATE TABLE IF NOT EXISTS bigrams (
    corpus TEXT NOT NULL,
    ngram TEXT NOT NULL CHECK(length(ngram) = 2),
    frequency REAL NOT NULL,
    PRIMARY KEY (corpus, ngram),
    FOREIGN KEY (corpus) REFERENCES corpus(corpus)
);

CREATE TABLE IF NOT EXISTS skipgrams (
    corpus TEXT NOT NULL,
    ngram TEXT NOT NULL CHECK(length(ngram) = 2),
    frequency REAL NOT NULL,
    PRIMARY KEY (corpus, ngram),
    FOREIGN KEY (corpus) REFERENCES corpus(corpus)
);

CREATE TABLE IF NOT EXISTS trigrams (
    corpus TEXT NOT NULL,
    ngram TEXT NOT NULL CHECK(length(ngram) = 3),
    frequency REAL NOT NULL,
    PRIMARY KEY (corpus, ngram),
    FOREIGN KEY (corpus) REFERENCES corpus(corpus)
);

CREATE TABLE IF NOT EXISTS settings (
    user_id INTEGER PRIMARY KEY,
    selected_corpus TEXT NOT NULL,
    FOREIGN KEY (selected_corpus) REFERENCES corpus(corpus)
);

CREATE TABLE IF NOT EXISTS layout (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    author TEXT NOT NULL,
    link TEXT,
    intended_corpus TEXT NOT NULL,
    FOREIGN KEY (intended_corpus) REFERENCES corpus(corpus),
    FOREIGN KEY (author) REFERENCES author(name)
);

CREATE TABLE IF NOT EXISTS author (
    name TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS likes (
    user_id INTEGER NOT NULL,
    layout_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, user_id),
    FOREIGN KEY (layout_id) REFERENCES layout(id)
);
