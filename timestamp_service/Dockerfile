FROM frolvlad/alpine-gcc

COPY target/fallback-cache-1.0.0-standalone.jar fallback-cache-1.0.0-standalone.jar

ENV PORT 8080

EXPOSE 8080

CMD ["cargo", "run", "--examples", "demo"]