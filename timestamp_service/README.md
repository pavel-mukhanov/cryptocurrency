# Timestamp sevice based on Exonum.

`cargo run --example demo`

* Add timestamp to data

`curl -H "Content-Type: application/json" -X POST -d @create_timestamp.json http://127.0.0.1:8000/api/services/timestamp_service/v1/timestamp`

* Get all timestamps

`curl -H "Content-Type: application/json" -X GET http://127.0.0.1:8000/api/services/timestamp_service/v1/timestamp/all`

* Get data timestamp

`curl -H "Content-Type: application/json" -X GET http://127.0.0.1:8000/api/services/timestamp_service/v1/timestamp/:data_hash`
