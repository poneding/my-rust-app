.PHONY:
release:
	git add .
	git commit --all --message "update"
	git push
	git tag -a $(VERSION) -m "release $(VERSION)"
	git push origin $(VERSION)
