.PHONY:
release:
	git add .
	git commit --all --message "update"
	git push
	export VERSION=$(cargo pkgid | awk -F'@' '{print $NF}')
	git tag -a $(VERSION) -m "release $(VERSION)"
	git push origin $(VERSION)
	cargo publish
