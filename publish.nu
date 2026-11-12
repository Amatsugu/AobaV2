def publish (tag: string) {
	docker build -f AobaServer/Dockerfile -t git.kaisei.app/amatsugu/aoba:($tag) -t git.kaisei.app/amatsugu/aoba:latest .
	docker push git.kaisei.app/amatsugu/aoba:($tag)
	docker push git.kaisei.app/amatsugu/aoba:latest
}
