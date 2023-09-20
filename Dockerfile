FROM rustlang/rust:nightly-alpine AS deps

# RUN echo "https://dl-cdn.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories
RUN apk update
RUN apk add musl-dev npm openssl-libs-static openssl-dev binaryen
RUN npm install -g sass

ARG LEPTOS_RUSTFLAGS=""
ENV OPENSSL_STATIC=1 OPENSSL_LIB_DIR=/usr/lib OPENSSL_INCLUDE_DIR=/usr/include/openssl RUST_BACKTRACE=1

ARG TARGET_CRATE=web
#ARG TARGETARCH
#
#RUN if [[ $TARGETARCH = "amd64" ]] ; then rustup target add x86_64-unknown-linux-musl ; \
#    else rustup target add aarch64-unknown-linux-musl ; fi
#RUN if [[ $TARGETARCH = "amd64" ]] ; then rustup toolchain install nightly-x86_64-unknown-linux-musl ; \
#    else rustup toolchain install nightly-aarch64-unknown-linux-musl ; fi

RUN rustup target add wasm32-unknown-unknown

ENV RUSTFLAGS=${LEPTOS_RUSTFLAGS}

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    if [[ ${TARGET_CRATE} == "web" ]] ; then cargo install cargo-generate ; fi
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    if [[ ${TARGET_CRATE} == "web" ]] ; then cargo install --features no_downloads --locked cargo-leptos ; fi

ENV RUSTFLAGS=""

WORKDIR /usr/src/mixtape

# Copy utility crates
COPY Cargo.lock ./
RUN printf "[workspace]\nmembers=[\"$TARGET_CRATE\"]\nresolver=\"2\"\n[profile.wasm-release]\ninherits = \"release\"\nopt-level = 'z'\nlto = true\ncodegen-units = 1\npanic = \"abort\"" > Cargo.toml

RUN USER=root cargo new --bin $TARGET_CRATE

# Build external libraries
WORKDIR /usr/src/mixtape/$TARGET_CRATE
RUN touch src/lib.rs
COPY $TARGET_CRATE/Cargo.toml .
# Clear all path-based (local) packages
RUN sed --in-place '/path = "\.\./d' Cargo.toml
#RUN if [[ $TARGETARCH = "amd64" ]] ; then cargo build --target x86_64-unknown-linux-musl --release ; \
#    else cargo build --target aarch64-unknown-linux-musl --release ; fi
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/mixtape/target \
    cargo build --release
# Try to separate cargo from everything else to reduce build times
#RUN mkdir www
#COPY $TARGET_CRATE/www/*.* ./www/
#WORKDIR /usr/src/mixtape/$TARGET_CRATE/www
#RUN if [[ -e "package.json" ]] ; then npm install ; fi

# Copy and build internal libraries
WORKDIR /usr/src/mixtape
COPY db ./db
COPY utils ./utils
COPY auth ./auth
COPY macros ./macros
RUN printf "[workspace]\nmembers=[\"db\",\"auth\",\"utils\",\"macros\",\"$TARGET_CRATE\"]\nresolver=\"2\"\n[profile.wasm-release]\ninherits = \"release\"\nopt-level = 'z'\nlto = true\ncodegen-units = 1\npanic = \"abort\"" > ./Cargo.toml

# Copy env file to all subdirectories (library crates)
COPY .env .
RUN find ./ -type d -path ./$TARGET_CRATE/assets -prune -exec cp .env {} \;

WORKDIR /usr/src/mixtape/$TARGET_CRATE
COPY $TARGET_CRATE/Cargo.toml ./Cargo.toml

#RUN if [[ $TARGETARCH = "amd64" ]] ; then cargo build --target x86_64-unknown-linux-musl --release \
#    else cargo build --target aarch64-unknown-linux-musl --release ; fi
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/mixtape/target \
    cargo build --release
RUN rm -r src
# END LIBRARIES

# Build executable
# Copy actual source files
COPY $TARGET_CRATE/ .
# Copy env file to all subdirectories (main crate)
RUN find . -type d -path ./$TARGET_CRATE/assets -prune -exec cp ../.env {} \;

ARG TEST_AND_RELEASE_CMD=cargo
FROM deps as tester

ARG MONGO_URI
ENV MONGO_TEST_DB_URI=$MONGO_URI

WORKDIR /usr/src/mixtape
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/mixtape/target \
    $TEST_AND_RELEASE_CMD test
WORKDIR /usr/src/mixtape/$TARGET_CRATE

FROM tester as builder

#RUN if [[ $TARGETARCH = "amd64" ]] ; then rm ../target/x86_64-unknown-linux-musl/release/deps/$TARGET_CRATE* ; \
#    else rm ../target/aarch64-unknown-linux-musl/release/deps/$TARGET_CRATE* ; fi
RUN --mount=type=cache,target=/usr/src/mixtape/target \
    rm -f ../target/release/deps/$TARGET_CRATE*

#RUN if [[ $TARGETARCH = "amd64" ]] ; then cargo build --target x86_64-unknown-linux-musl --release ; \
#    else cargo build --target aarch64-unknown-linux-musl --release ; fi
RUN mkdir ../out
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/mixtape/target \
    $TEST_AND_RELEASE_CMD build --release; if [[ ${TARGET_CRATE} == "web" ]] ; then mv ../target/server/release/$TARGET_CRATE ../out/app && mv ../site ../out/ ; else mv ../target/release/$TARGET_CRATE ../out/app ; fi;
#RUN if [[ $TARGETARCH = "amd64" ]] ; then mv ../target/x86_64-unknown-linux-musl/release/$TARGET_CRATE ../target/release/app ; \
#    else mv ../target/aarch64-unknown-linux-musl/release/$TARGET_CRATE ../target/release/app; fi
#RUN --mount=type=cache,target=/usr/src/mixtape/target test -x ../target/release/$TARGET_CRATE
#RUN --mount=type=cache,target=/usr/src/mixtape/target \
#    mv ../target/release/$TARGET_CRATE ../out/app
# WORKDIR /usr/src/mixtape/$TARGET_CRATE/www
# RUN if [[ -e "package.json" ]] ; then npm run build ; fi
# RUN if [[ -e "package.json" ]] ; then mkdir -p ../../out/dist ; fi
# RUN if [[ -e "package.json" ]] ; then mv dist/* ../../out/dist ; fi

FROM alpine AS webserver

ARG APP=/usr/src
ENV RUST_BACKTRACE=1

EXPOSE 80
EXPOSE 27017
#
#ENV TZ=Etc/UTC
##    APP_USER=appuser
##
##RUN addgroup -S $APP_USER \
##    && adduser -S -g $APP_USER $APP_USER
#
#RUN apk update \
#    && apk add --no-cache musl-dev openssl-dev musl openssl musl-utils \
#    && rm -rf /var/cache/apk/*
RUN apk update
RUN apk add --no-cache ca-certificates

COPY --from=builder /usr/src/mixtape/out/ ${APP}/tmp

#RUN chown -R $APP_USER:$APP_USER ${APP}

#USER $APP_USER
WORKDIR ${APP}
RUN mv tmp/app .
RUN if [[ -d "tmp/site" ]] ; then cp -r ./tmp/site/ ./site ; fi
RUN rm -r tmp

# Safety precaution
RUN find . -name .env -type f -delete

ENTRYPOINT ["./app"]