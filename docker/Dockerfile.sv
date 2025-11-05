# syntax=docker/dockerfile:1

# Dockerfile for the svelte frontend

FROM node:24.3-alpine3.21 as base
WORKDIR /app
COPY web-ui/package*.json ./
RUN npm ci
RUN npx svelte-kit sync

FROM base as dev
EXPOSE 3000

COPY web-ui .

VOLUME [ "/app/src" ]
VOLUME [ "/app/static" ]

ENTRYPOINT ["npm", "run"]
CMD [ "dev", "--", "--host", "0.0.0.0" ]

FROM base as build
COPY web-ui .
RUN npm run build

FROM nginx:1.29.0-alpine as prod
COPY --from=build /app/build /usr/share/nginx/html
COPY gateway/nginx-prod.conf /etc/nginx/nginx.conf
COPY gateway/inbound /etc/nginx/inbound
COPY gateway/outbound /etc/nginx/outbound

EXPOSE 3000