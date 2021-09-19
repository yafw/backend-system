openssl ecparam -name prime256v1 -genkey -noout -out private-key.pem
openssl ec -in private-key.pem -pubout -out public-key.pem
openssl req -new -x509 -key private-key.pem -out cert.pem -days 1825 -subj "/C=US/ST=NA/L=BackendSystem/O=BackendSystem/CN=localhost"
openssl pkcs8 -topk8 -nocrypt -in private-key.pem -out private-key-pkcs8.pem
