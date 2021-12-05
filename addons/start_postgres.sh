# Run
docker run --name mypostgres -d -p 15432:5432 -e POSTGRES_PASSWORD=123456 postgres:14.1

# Test
docker exec -it mypostgres psql -U postgres -d postgres

# -- select * from pg_tables;
