# read_data_list
curl -XGET -H 'Content-Type: application/json' -H "authorization:MTIzNDU2" http://localhost:9527/api/url_maps

# read_data_by_id
curl -XGET -H 'Content-Type: application/json' -H "authorization:MTIzNDU2" http://localhost:9527/api/url_maps/qq

# create_data
curl -XPOST -H 'Content-Type: application/json' -H "authorization:MTIzNDU2" -d '{"key": "github", "url": "github.com"}' http://localhost:9527/api/url_maps/

# update_data
curl -XPUT -H 'Content-Type: application/json' -H "authorization:MTIzNDU2" -d '{"key": "github", "url": "fake.github.com"}' http://localhost:9527/api/url_maps/github

# delete_data_by_id
curl -XDELETE -H 'Content-Type: application/json' -H "authorization:MTIzNDU2" http://localhost:9527/api/url_maps/github
