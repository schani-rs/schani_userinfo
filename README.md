# schani_userinfo

## Development setup

Start postgres database server:
```bash
docker run --name schani-userinfo-postgres -e POSTGRES_USER=schani_userinfo -e POSTGRES_PASSWORD=user -e POSTGRES_DB=password -p 543
2:5432 -d postgres
```
