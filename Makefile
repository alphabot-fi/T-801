.PHONY: all build proto

BIN_DIR := ./bin
version := $(shell git rev-parse --short=12 HEAD)
timestamp := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

sdbdir=/tmp/zgranx/sdb
sdbinitdir="$(ROOT_DIR)/deployments/sdb"


all: build


build: proto
	rm -f $(BIN_DIR)/exaftx*
	go build -o $(BIN_DIR)/exaftx_server -v -ldflags \
  "-X main.rev=$(version) -X main.bts=$(timestamp)" cmd/exaftx/server/main.go
	go build -o $(BIN_DIR)/exaftx_client -v -ldflags \
  "-X main.rev=$(version) -X main.bts=$(timestamp)" cmd/exaftx/client/main.go


proto:
	$(shell find internal/ | grep pb.go$ | xargs rm -f)
	protoc --go_out=. --go-grpc_out=. api/monitor.proto
	protoc --experimental_allow_proto3_optional --go_out=. --go-grpc_out=. api/exa.proto


dockerinit:
	-docker container prune -f >/dev/null 2>&1
	-docker network create zgranxnet >/dev/null 2>&1


sdbinit: sdbhalt
	-docker container prune -f >/dev/null 2>&1
	-sudo rm -rf $(sdbdir)
	-docker run --detach -v $(sdbdir):/var/lib/mysql:z  -v $(sdbinitdir):/docker-entrypoint-initdb.d:z --network zgranxnet --name szgranxdb --env MARIADB_USER=$(ZGRANX_SDB_USER) --env MARIADB_PASSWORD=$(ZGRANX_SDB_PASSWORD) --env MARIADB_ROOT_PASSWORD=$(ZGRANX_SDB_ROOT_PASSWORD) --env MARIADB_DATABASE=$(ZGRANX_SDB_DATABASE) mariadb:latest


sdbstart:
	-docker container prune -f >/dev/null 2>&1
	-docker run --detach -v $(sdbdir):/var/lib/mysql  --network zgranxnet --name szgranxdb --env MARIADB_USER=$(ZGRANX_SDB_USER) --env MARIADB_PASSWORD=$(ZGRANX_SDB_PASSWORD) --env MARIADB_ROOT_PASSWORD=$(ZGRANX_SDB_ROOT_PASSWORD) mariadb:latest


sdbhalt:
	-docker stop szgranxdb
	-docker container prune -f >/dev/null 2>&1

sdbprompt:
	-docker container prune -f >/dev/null 2>&1
	-docker run --network zgranxnet -it --rm mariadb mysql -h szgranxdb -u $(ZGRANX_SDB_USER) -D $(ZGRANX_SDB_DATABASE) -p$(ZGRANX_SDB_PASSWORD)