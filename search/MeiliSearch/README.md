* https://github.com/meilisearch/MeiliSearch

```sh
git clone https://github.com/meilisearch/MeiliSearch.git
cd MeiliSearch
cargo run --release
```

```sh
# download sample data
curl -L 'https://bit.ly/2PAcw9l' -o movies.json

# crate index
curl -i -X POST 'http://127.0.0.1:7700/indexes' --data '{ "name": "Movies", "uid": "movies" }'

# index documents
curl -i -X POST 'http://127.0.0.1:7700/indexes/movies/documents' \
  --header 'content-type: application/json' \
  --data-binary @movies.json

# search
curl 'http://127.0.0.1:7700/indexes/movies/search?q=botman+robin&limit=2' | jq
```
