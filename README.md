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

## Advanced Search parser
```
({kind}:{search_single_word}\ {second_world} {kind}!:"{search with spaces}")
```
- '!' => not operator/exclude
- valid groups => ["title", "t"]
- strings with spaces needs to wrapped with ""
- '\' => escape parser relevant symbols outside "" or " inside a string
- groups are wrapped with () [tags will be connected with and]
- nesting groups is possible
- valid group prefixes => [":or(...)", ":and(...)"]

Example:
this will search for items where the title does not contain "some title", but "other title".
The title also needs to contain "part1" or "part2"
```
and:(title:!"some title" title:"other title" or:("part1" "part2"))
```