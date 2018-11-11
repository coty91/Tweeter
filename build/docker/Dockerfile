FROM shepmaster/rust-nightly:latest AS tweeter_build

RUN mkdir /home/tweeter

ADD . /home/tweeter

WORKDIR /home/tweeter

RUN BACKTRACE=1 cargo build --release

RUN mv target/release/tweeter /bin/tweeter