# ManRead API

Part of [ManRead](https://github.com/ManReadApp/ManRead)

### Custom Data Path
- change `root_folder: new_path` in config.yml
- move the spinners folder from `data` to `new_path`

### Register external icons 
- create filter file(utf-8) with a name pattern like this [uri].filter
- Example: `asuratoon.filter`
- add filters in the file. valid patterns are `starts_with [str]`, `end_with [str]`, `contains [str]`, `regex [str]`
- Example: `contains asuratoon.com` or `ends_with ?page=1` or `starts_with https://asuratoon.com`
- add img to folder with naming pattern [uri].[ext]
- Example: `asuratoon.ico`
- The uri from the filter must match the icon uri
