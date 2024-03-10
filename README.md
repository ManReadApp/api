# ManRead API

Part of [ManRead](https://github.com/ManReadApp/ManRead)

## Custom Data Path
- change `root_folder: new_path` in config.yml
- move the spinners folder from `data` to `new_path`

## External Sites
files need to be in [root_folder]/external

### Register external icons
- create filter file(utf-8) with a name pattern like this [uri].filter
- Example: `asuratoon.filter`
- add filters in the file. valid patterns are `starts_with [str]`, `end_with [str]`, `contains [str]`, `regex [str]`
- Example: `contains asuratoon.com` or `ends_with ?page=1` or `starts_with https://asuratoon.com`
- add img to folder with naming pattern [uri].[ext]
- Example: `asuratoon.ico`
- The uri from the filter must match the icon uri
Scrape files need to have a .scrape ext

### Usefull tools for svg icons
- [svg-path-editor](https://yqnn.github.io/svg-path-editor/)
- [svg-crop](https://svgcrop.com/)

### Register scraper
Files need to have the `.scrape` ext

Example:
```
"kind": "Metadata", "request_config": "headers.json"
image[href] div div#test.est2 ... img.cover
```

### Header structure
- uri is required
- kind is required and can be ['SingleSiteScraper', 'MultiSiteScraper', 'Search', 'Metadata']
- request_config is optional and points to a json file with request headers

### Selector Line structure
- field can contain letter, number or _
- [] is the value that will be extracted
- valid values are `href`, `text`, `html`, `src`, `attr=custom`,
- @ prefix gets all items
- selector = like document.querySelectorAll()