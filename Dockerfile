FROM jimmycuadra/rust:latest

EXPOSE 4000
RUN mkdir /haxonite
WORKDIR /haxonite
ENV PATH="/root/.cargo/bin:${PATH}"

ENV HAXONITE_VERSION=0.1.0

RUN cargo install haxonite --vers $HAXONITE_VERSION

CMD haxonite

# Run with:
# docker run --rm -ti -v /host/directory/haxonite:/haxonite jmatraszek/haxonite
