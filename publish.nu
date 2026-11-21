def main (tag: string) {
	docker build -f AobaServer/Dockerfile -t git.kaisei.app/amatsugu/aoba:($tag) .
}
