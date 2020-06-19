V2 4:00

**Youtube**

https://youtu.be/gQwA0g0NNSI

**Notes**

systemfd --no-pid -s http::8080 -- cargo watch -x run

watchexec --restart "cargo run"
curl http://localhost:8080/todos

docker-compose up -d
psql -h 127.0.0.1 -p 5432 -U <user> <dbname>
psql -h localhost -p 5432 -U postgres
CREATE DATABASE <db_name>;
psql -h 127.0.0.1 -p 5432 -U <user> <dbname> < database.sql
