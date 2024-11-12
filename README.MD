# QUICK-TRADER
=======================
# SSH
ssh -i C:\Users\mingh\.ssh\PC01.pem root@43.156.20.92


=======================
docker rm $(docker ps -aq)
docker rmi $(docker images -q)
=======================
# DOCKER
FROM python:3.10-slim

ENV PYTHON_PATH /usr/local/bin/python
ENV PIP_PATH /usr/local/bin/pip

RUN apt update && apt install build-essential wget git gcc curl libssl-dev pkg-config -y && cd ~/ && wget http://prdownloads.sourceforge.net/ta-lib/ta-lib-0.4.0-src.tar.gz && tar -xzf ta-lib-0.4.0-src.tar.gz && cd ta-lib && ./configure -prefix=/usr && make && make install  && pip install numpy==1.23.1 && pip install ta-lib && pip install pandas==1.5

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

ENV GIT_PATH /usr/bin/git

CMD ["sh", "-c", "if [ -d /root/binance/quick-trader/.git ]; then cd /root/binance/quick-trader && git pull && cargo build; else git clone git@github.com:mark-he/quick-trader.git /root/binance/quick-trader; fi && if [ -d /root/binance/rust-conn/.git ]; then cd /root/binance/rust-conn && git pull && python bn_run.py; else git clone git@github.com:mark-he/rust-conn.git /root/binance/rust-conn; fi && tail -f /dev/null"]




CMD ["/bin/bash"]

docker build -f rust.dockerfile -t rust .
docker run --name rust -it -v /root/.ssh/id_rsa:/root/.ssh/id_rsa rust

# Connect
docker start python
docker exec -it python bash





CMD ["sh", "-c", "git clone git@github.com:mark-he/rust-conn.git /home/ec2-user/binance/rust-conn && if [ -d /home/ec2-user/binance/rust-conn/.git ]; then cd /home/ec2-user/binance/rust-conn && git pull; fi"]
