FROM clux/muslrust:stable as build
COPY . /build
RUN cd /build && cargo build --release

FROM scratch
COPY --from=build /build/target/*/release/azuma_backend /azuma_backend
ENTRYPOINT ["/azuma_backend"]