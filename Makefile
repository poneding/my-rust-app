VERSION := $(shell cargo pkgid | awk -F'@' '{print $$NF}')
export VERSION

.PHONY:
all: commit release

.PHONY:
commit:
	git add .
	git commit --all --message "update"
	git push

.PHONY:
release:
	git tag -a v$(VERSION) -m "release v$(VERSION)"
	git push origin v$(VERSION)
	cargo publish

.PHONY:
check:
	echo $(VERSION)