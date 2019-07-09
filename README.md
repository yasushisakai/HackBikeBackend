# hackbike backend

# Data Structure
```json
{
  "app_id"      : "", // String
  "start_ts"    : ,   // Number
  "end_ts"      : ,   // Number
  "coordinates" :[
		{"lat":,"lon":,"ts":},  // Number, Number, Number
		{}, ..., {}
	],
  "bike_data"   : "" // String
}
```

# Directory

```
root/
    ├ src/
    |   ├ handlers.rc
    |   └ main.rc
    ├ Cargo.toml
    |
    ├ database/
    |   ├ {app_id}/
    |   |       ├ {app_id}_{start_ts}.json
    |   |       ├ {app_id}_{start_ts}.json
    |   |       :
    |   |       └ {app_id}_{start_ts}.json
    |   ├ {app_id}/
    |   |       ├ {app_id}_{start_ts}.json
    |   :       :
    |
    └ html/

```

