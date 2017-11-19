# schani_userinfo

## API

### Authenticate

Verify a username-password combination. The password is specified as `password` POST parameter.

``POST /user/<username>/authenticate``

```bash
curl -X POST localhost:8000/user/user1/authenticate -d password=pwd1
```

### Save user setting

Save a new key-value based setting.

``PUT /user/<user_id>/setting``

```bash
curl -X PUT localhost:8000/user/1/setting -d key=test -d value=val
```

### Get user setting

Retrieve the value of a setting.

``GET /user/<user_id>/setting/<key>``

```bash
curl localhost:8000/user/1/setting/test
```

## Development setup

Start postgres database server:
```bash
docker run --name schani-userinfo-postgres -e POSTGRES_USER=schani_userinfo -e POSTGRES_PASSWORD=password -e POSTGRES_DB=schani_userinfo -p 5432:5432 -d postgres
```
