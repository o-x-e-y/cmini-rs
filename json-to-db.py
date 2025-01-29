import json
import sqlite3
import glob

DB_PATH = "./test.db"

def migrate_json_to_db(json_path, db_path):
    with open(json_path) as f, sqlite3.connect(db_path) as conn:
        data = json.load(f)
        corpus = data['name']
        
        # Insert language metadata
        conn.execute('''
            INSERT INTO corpus VALUES
            (?, ?, ?, ?, ?)
        ''', (
            corpus,
            data['char_total'],
            data['bigram_total'],
            data['skipgram_total'],
            data['trigram_total']
        ))
        
        # Helper function for batch inserts
        def batch_insert(table, items):
            conn.executemany(f'''
                INSERT INTO {table}
                VALUES (?, ?, ?)
            ''', [(corpus, ngram, freq) for ngram, freq in items])
        
        # Insert all n-gram types
        batch_insert('chars', data['chars'].items())
        batch_insert('bigrams', data['bigrams'].items())
        batch_insert('skipgrams', data['skipgrams'].items())
        batch_insert('trigrams', data['trigrams'].items())


if __name__ == "__main__":
    for corpus in glob.glob("./corpora/*.json"):
        print(corpus)

        migrate_json_to_db(corpus, DB_PATH)
