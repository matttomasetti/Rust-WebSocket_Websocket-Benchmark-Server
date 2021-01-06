FROM ubuntu

# Set timezone and environment variables
ENV TZ=America/New_York
ENV PATH="/root/.cargo/bin:${PATH}"
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Add source files to docker image
ADD .	/home/websocket

# Use bash rather than sh
SHELL ["/bin/bash", "-c"] 

# Update and install dependencies
RUN apt-get -y update \
    && apt-get -y upgrade \
    && apt-get -y install curl build-essential pkg-config libssl-dev ksh\
    && export KSH_VERSION='2020.0.0'

# Install rust
RUN cd /home/websocket \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >> install.sh \
    && chmod +x install.sh \
    && ./install.sh -y \
    && source /root/.cargo/env \

# Build project
RUN cargo build

    
EXPOSE 8080

WORKDIR /home/websocket
CMD ["/root/.cargo/bin/cargo", "run"]
