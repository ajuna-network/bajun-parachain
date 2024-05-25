FROM phusion/baseimage:jammy-1.0.1
LABEL description="This is the 2nd stage: a very small image where we copy the node binary."

RUN apt-get update && \
apt-get install -y jq


RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	useradd -m -u 1000 -U -s /bin/sh -d /ajuna ajuna && \
	mkdir -p /ajuna/.local/share/bajun-node && \
	chown -R ajuna:ajuna /ajuna/.local && \
	ln -s /ajuna/.local/share/bajun-node /data

COPY bajun-node /usr/local/bin
COPY ./scripts/healthcheck9933.sh /usr/local/bin

RUN chmod +x /usr/local/bin/bajun-node
RUN chmod +x /usr/local/bin/healthcheck9933.sh

# checks
RUN ldd /usr/local/bin/bajun-node && \
	/usr/local/bin/bajun-node --version

# Shrinking
#RUN rm -rf /usr/lib/python* && \
#	rm -rf /usr/bin /usr/sbin /usr/share/man

USER ajuna

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/bajun-node"]
