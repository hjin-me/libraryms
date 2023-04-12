FROM rust:latest as builder
RUN apt-get update && apt-get install -y librust-clang-sys-dev
ENV CARGO_HOME /build/.cargo
WORKDIR /build
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --target-dir /output --release

# debian release as the same as golang image
# set TimeZone as Asia/Shanghai
# set Local as zh-hans
FROM debian:bullseye
RUN set -ex; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
	    tzdata \
	    locales \
	    ca-certificates;
RUN locale-gen zh_CN.UTF-8; \
    update-locale zh_CN.UTF-8;
RUN cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime;
ENV TZ Asia/Shanghai
ENV LANG zh_US.utf8
COPY --from=builder /output/release/libraryms /usr/local/bin/libraryms
ENTRYPOINT ["libraryms"]
