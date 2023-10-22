FROM alpine:3.14
RUN apk add --update nodejs nodejs-npm curl gcc
RUN "curl -proto '=https' -tlsv1.2 -sSf https://sh.rustup.rs | sh"
COPY . vilchain
CMD [ "cd", "vilchain" ]