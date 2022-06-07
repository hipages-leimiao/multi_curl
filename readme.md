# Multi_curl

## query payment user status as csv

```sql
select client_user_id,status from users
```

save as attributes.csv

## cvt to json

```shell
sh csv_to_json.sh
```

## run braze attributes migration

> Will run request in concurrency as 2, and chunk attributes per 75(base on api limitation per request)

```shell
# add braze api config in .env
cargo run
```
