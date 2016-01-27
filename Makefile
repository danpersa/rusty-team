default:
	set -xe
	docker build --rm -t zalando/rusty-team-build .
	docker run --rm -v /var/run/docker.sock:/var/run/docker.sock -ti zalando/rusty-team-build

# run this as soon as you add, remove or modify one of the dependencies of the project
build-base-image:
	docker build -f Dockerfile.base --rm -t zalando/rusty-team-build-base .

build-all: build-base-image default

docker-run:
	docker run -p 6768:6768 -e RUST_LOG=info -t danpersa/rusty-team:latest

docker-push:
	docker push danpersa/rusty-team:latest

# virtual box port forwarding
# VBoxManage controlvm "default" natpf1 "tcp-port6768,tcp,,6768,,6768"
curl:
	curl -v http://localhost:6768/api/teams\?member\=teams-team1,virtual-team2,official

bench:
	wrk -t12 -c900 -d10s http://localhost:6768/api/teams\?member\=teams-team1,virtual-team2,official
