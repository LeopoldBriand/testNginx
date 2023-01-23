FROM node:lts-alpine

COPY ./* ./

RUN npm ci --only=production

EXPOSE 8090

CMD ["npm", "run", "start"]