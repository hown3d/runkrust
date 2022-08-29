oci-test:
	cd runtime-tools; \
	make runtimetest validation-executables \
	sudo make RUNTIME=target/debug/runkrust localvalidation
