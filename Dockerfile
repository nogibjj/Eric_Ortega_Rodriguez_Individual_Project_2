FROM alpine:latest
WORKDIR /app

# Remove or replace this line if repeat.sh is not required
# COPY repeat.sh /app

RUN apk update && apk add bash
