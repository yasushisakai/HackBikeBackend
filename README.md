# hackbike backend

# Usage

## Pushing data from client
```
curl -X POST -d '{JSON DATA}' server_address/api/data
```

## Checking app_id list from clent
access to `server_address/api/data`

## Checking each app_id data from clent
access to `server_address/api/data/{app_id}`

## File transfer
```
curl -X POST -H "Content-Type: application/octet-stream" --data-binary "@{upload_file_path}" server_address/api/file/{save_filename.ext}
```


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

# Daemon Service Setup
## Creating service script
/etc/systemd/system/hackbike.service
```
[Unit]
Description=Hackable Bike Backend

[Service]
WorkingDirectory=/path/to/HackBikeBackend
ExecStart=/path/to/executable/hackbike_backend
Restart=always
Type=simple

[Install]
WantedBy=multi-user.target
```
## Instructions
```bash
# reload daemon
sudo systemctl daemon-reload
# start
sudo systemctl start hackbike
# check run status
systemctl status hackbike
# stop
sudo systemctl stop hackbike
# add to startup service
sudo systemctl enable hackbike
# check startup status
systemctl is-enabled hackbike
```
