FROM ubuntu:24.04

RUN apt-get update
RUN apt-get -y install nano git cron curl build-essential pkg-config libssl-dev libsqlite3-dev

#
# install cargo et al
#
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf > /tmp/rust_installer
RUN /usr/bin/sh /tmp/rust_installer -y

#
# build fork stats binary
#
WORKDIR /app
RUN --mount=type=bind,source=stats,target=/build \
    cd /build && \
    /root/.cargo/bin/cargo build --target-dir=/tmp/build && \
    cp /tmp/build/debug/fork /app/fork

COPY update_stats.sh .

#
# set-up repositories
#
WORKDIR /repos

RUN git clone --no-checkout --origin maxiv https://gitlab.maxiv.lu.se/kits-maxiv/mxcube/mxcube3.git mxcubeweb
RUN cd mxcubeweb && git remote add upstream https://github.com/mxcube/mxcubeweb.git

RUN git clone --no-checkout --origin maxiv https://gitlab.maxiv.lu.se/kits-maxiv/mxcube/mxcubecore.git
RUN cd mxcubecore && git remote add upstream https://github.com/mxcube/mxcubecore.git

#
# configure cron to run our fork stat updates script
#
COPY cronjob /tmp
RUN /usr/bin/crontab -u root /tmp/cronjob


WORKDIR /app

CMD ["cron", "-f"]
