# A disposable SSH server for local development and integration tests.
# Never expose it beyond localhost: the test user has a known password.
FROM alpine:3.22

RUN apk add --no-cache openssh bash \
  && adduser -D -s /bin/bash sallygate \
  && echo 'sallygate:sallygate-test-password' | chpasswd \
  && sed -i \
    -e 's/^#\?PasswordAuthentication .*/PasswordAuthentication yes/' \
    -e 's/^#\?PermitRootLogin .*/PermitRootLogin no/' \
    -e 's/^#\?AllowTcpForwarding .*/AllowTcpForwarding yes/' \
    /etc/ssh/sshd_config \
  # Tests drop connections before login on purpose (unknown server key); do not penalize them.
  && echo 'PerSourcePenalties no' >> /etc/ssh/sshd_config

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 22
ENTRYPOINT ["/entrypoint.sh"]
