# schani_userinfo

## API

### Authenticate

Verify a username-password combination. The password is specified as `password` POST parameter.

``POST /user/<username>/authenticate``

```bash
curl -X POST localhost:8000/user/user1/authenticate -d password=pwd1
```

## Development setup

Start postgres database server:
```bash
docker run --name schani-userinfo-postgres -e POSTGRES_USER=schani_userinfo -e POSTGRES_PASSWORD=user -e POSTGRES_DB=password -p 543
2:5432 -d postgres
```
