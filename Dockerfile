FROM node:20.10.0-alpine AS builder

RUN apk add git

USER node

# Create app directory (with user `node`)
RUN mkdir -p /home/node/app
# Set is as cwd
WORKDIR /home/node/app

# Install app dependencies
# A wildcard is used to ensure both package.json AND package-lock.json are copied
# where available (npm@5+)
COPY --chown=node package*.json ./

# Install dependencies
RUN npm install

# Bundle app source code
COPY --chown=node . .
COPY --chown=node ./.git ./.git

RUN npm run build

FROM nginx:1.25.3-alpine3.18-slim

WORKDIR /etc/nginx/conf.d

COPY --from=builder /home/node/app/build /home/node/app

COPY ./server.nginx ./default.conf
