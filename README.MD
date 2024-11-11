# QUICK-TRADER

## 1. How to test
1) If you are going to test CTP, you need to copy the cooresponding lib in the folder of ctp/docs/lib/ to the folder of target/debug or target/release.

2) At the root path, execute "cargo test  test_backtest_trade_gateway --package unit_test"

# Docker file - rust
FROM rust:latest
VOLUME /data /home/ec2-user/binance/data
RUN apt-get update && apt-get install -y vim
RUN apt-get update && apt-get install -y git
ENV GIT_PATH /usr/bin/git

CMD ["sh", "-c", "git clone git@github.com:mark-he/quick-trader.git /root/quick-trader"]
CMD ["sh", "-c", "cd /root/quick-trader && git pull;"]
CMD ["sh", "-c", "cd /root/quick-trader && cargo build --release;"]
CMD ["/bin/bash"]

git clone git@github.com:mark-he/quick-trader.git /home/ec2-user/binance/quick-trader && if [ -d /home/ec2-user/binance/quick-trader/.git ]; then cd /home/ec2-user/binance/quick-trader && git pull; fi

# Build
docker build -f rust.Dockerfile -t rust .

# Run
docker run --name rust -it -v /home/ec2-user/.ssh/id_rsa:/root/.ssh/id_rsa rust

# Connect
docker start rust
sudo docker exec -it rust bash

# Dockerfile

FROM python:3.10-slim-buster
VOLUME /data /home/ec2-user/binance/data

ENV PYTHON_PATH /usr/local/bin/python
ENV PIP_PATH /usr/local/bin/pip

RUN apt-get update && apt-get install -y git && apt-get install -y gcc
ENV GIT_PATH /usr/bin/git


CMD ["/bin/bash"]

CMD ["sh", "-c", "git clone git@github.com:mark-he/rust-conn.git /home/ec2-user/binance/rust-conn && if [ -d /home/ec2-user/binance/rust-conn/.git ]; then cd /home/ec2-user/binance/rust-conn && git pull; fi"]

# Build
docker build -f python.Dockerfile -t python .

# Run
docker run --cpus=".5" --memory="1024m" --name python -it -v /home/ec2-user/.ssh/id_rsa:/root/.ssh/id_rsa -v /home/ec2-user/binance/data:/data python
docker run -d --name python python
git clone git@github.com:mark-he/rust-conn.git /root/rust-conn && git pull

# Connect
docker start python
docker exec -it python bash

# 删除所有容器
docker rm $(docker ps -aq)
docker rmi $(docker images -q)

18.141.164.160


git clone git@github.com:mark-he/quick-trader.git /home/ec2-user/binance/quick-trader && if [ -d /home/ec2-user/binance/quick-trader/.git ]; then cd /home/ec2-user/binance/quick-trader && git pull; fi
git clone git@github.com:mark-he/rust-conn.git /home/ec2-user/binance/rust-conn && if [ -d /home/ec2-user/binance/rust-conn/.git ]; then cd /home/ec2-user/binance/rust-conn && git pull; fi
ssh -i D:\workspace\aws\AWS01.pem ec2-user@ec2-18-141-164-160.ap-southeast-1.compute.amazonaws.com