# Start postgres
kubectl apply -f postgres.yml

# Create Database[in container]
psql -U postgres -d postgres

# Execute sql in schema

# Start Service
kubectl apply -f url-mapper-rs.yml

# Test
curl -XGET -H 'Content-Type: application/json' -H "authorization:MTIzNDU2" http://<cluster-ip>:9527/api/url_maps

# Print:
#
# [{"key":"qq","url":"qq.com"},{"key":"google","url":"google.com"},{"key":"facebook","url":"facebook.com"},{"key":"twitter","url":"twitter.com"},{"key":"bilibili","url":"bilibili.com"}]
