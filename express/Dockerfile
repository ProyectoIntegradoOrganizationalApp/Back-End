FROM node

WORKDIR /app

COPY ./package.json .

RUN npm install -g nodemon && npm install

COPY . .

CMD ["nodemon"]