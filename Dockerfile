FROM rust as builder

WORKDIR /usr/src

RUN git clone https://github.com/avu12/first_rust.git
WORKDIR /usr/src/first_rust
RUN cargo install --path .


FROM ubuntu
EXPOSE 8080
RUN apt-get update
# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl 
COPY --from=builder /usr/local/cargo/bin/first_rust /usr/local/bin/first_rust
#Workarond for the html page:
COPY --from=builder /usr/src/first_rust/index.html /index.html
CMD ["first_rust"]