oci-test:
	docker build -f test/Dockerfile -t runkrust-oci-test .
	docker run runkrust-oci-test

firecracker-client-lib:
	sh scripts/firecracker-client-gen.sh