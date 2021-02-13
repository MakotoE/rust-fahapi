FROM rust:1.50-buster

RUN apt update && apt install -y wget curl bzip2
RUN rustup component add rustfmt clippy
RUN wget https://download.foldingathome.org/releases/public/release/fahclient/debian-stable-64bit/v7.6/fahclient_7.6.21_amd64.deb
RUN dpkg -i fahclient_7.6.21_amd64.deb
RUN FAHClient --version