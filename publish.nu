def publish (tag: string) {
	docker build -f AobaServer/Dockerfile -t git.kaisei.app/amatsugu/aoba:($tag) .; docker push git.kaisei.app/amatsugu/aoba:($tag)
}
