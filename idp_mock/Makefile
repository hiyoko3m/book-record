.PHONY:\
	run\
	build\
	stop\

run:
	docker run -v $(PWD)/creds:/tmp/idp_mock --name idp_mock_server \
		-p 8001:80 \
		-e BASE_URL=http://localhost:1823 \
		-e PORT=80 \
		-e RSA_PEM_FILE=/tmp/idp_mock/rsa_key.pem \
		-d idp_mock 

build:
	docker build -t idp_mock:latest .

stop:
	docker stop idp_mock_server
	docker rm idp_mock_server
