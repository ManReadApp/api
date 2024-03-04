# ManRead App

Part of [ManRead](https://github.com/ManReadApp/ManRead)

## Run web
```sh
trunk serve

open http://127.0.0.1:8079/
```

## Set custom server(Client)
Set in url
```
open http://127.0.0.1:8079/?server=http://127.0.0.1:8080
```

## Set custom server(Server)
Add this to head in index.html
```
<script>var server_url = 'http://127.0.0.1:8082';</script>
```