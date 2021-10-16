FROM mcr.microsoft.com/vscode/devcontainers/base:0-focal
ARG NONROOT_USER=vscode

# Install required programs for the container
RUN apt update -y && apt install libssl-dev pkg-config apt-transport-https curl sudo gnupg python build-essential ca-certificates lsb-release -y && \
    # Add bazel repository gpg keys
    curl -fsSL https://bazel.build/bazel-release.pub.gpg | gpg --dearmor -o /etc/apt/trusted.gpg.d/bazel.gpg && \
    # Add docker repository gpg keys
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg && \
    # Add docker repository apt source
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list && \
    # Add bazel repository apt source
    echo "deb [arch=amd64] https://storage.googleapis.com/bazel-apt stable jdk1.8" | tee /etc/apt/sources.list.d/bazel.list && \
    # Install bazel tools
    curl -L https://github.com/bazelbuild/buildtools/releases/download/4.2.0/buildifier -o /usr/bin/buildifier && \
    curl -L https://github.com/bazelbuild/buildtools/releases/download/4.2.0/buildozer-linux-amd64 -o /usr/bin/buildozer && \
    chmod +x /usr/bin/buildifier && chmod +x /usr/bin/buildozer && \
    # Install docker & bazel
    apt update -y && apt install bazel docker-ce-cli -y

# Add the user to the sudo group
RUN adduser $NONROOT_USER sudo
# Allow to use sudo without password
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Startup script, configure docker for the container non-root user
RUN echo "#!/bin/sh\n\
    . ~/.cargo/env\n\
    SOCKET_GID=\$(stat -c '%g' /var/run/docker.sock) \n\
    if [ \"${SOCKET_GID}\" != '0' ]; then\n\
        if [ \"\$(cat /etc/group | grep :\${SOCKET_GID}:)\" = '' ]; then sudo groupadd --gid \${SOCKET_GID} docker-host; fi \n\
        if [ \"\$(id ${NONROOT_USER} | grep -E \"groups=.*(=|,)\${SOCKET_GID}\(\")\" = '' ]; then sudo usermod -aG \${SOCKET_GID} ${NONROOT_USER}; fi\n\
    fi\n\
    exec \"\$@\"" > /usr/local/share/docker-init.sh \
    && chmod +x /usr/local/share/docker-init.sh

USER $NONROOT_USER

# Install go & rust toolchains
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y && \
    curl https://raw.githubusercontent.com/canha/golang-tools-install-script/master/goinstall.sh | bash

ENTRYPOINT [ "/usr/local/share/docker-init.sh" ]
# Required for vscode to start the server inside the container
CMD [ "sleep", "infinity" ]
