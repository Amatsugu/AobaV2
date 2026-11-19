def publish (tag: string, publish: bool = false) {
	docker build -f AobaServer/Dockerfile -t git.kaisei.app/amatsugu/aoba:($tag) -t git.kaisei.app/amatsugu/aoba:latest .
	if publish {
		docker push git.kaisei.app/amatsugu/aoba:($tag)
		docker push git.kaisei.app/amatsugu/aoba:latest
	}
}
