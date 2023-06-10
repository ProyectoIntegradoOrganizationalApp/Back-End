// Express
const express = require('express');
const app = express();

// Recogemos la dependencia
const ImageKit = require('imagekit');

// Creamos una instancia de image kit con los datos de la API
const imagekit = new ImageKit({
    urlEndpoint: 'https://ik.imagekit.io/iqaq9l86z',
    publicKey: 'public_w9qPiTYJuv7CC8eoihguu1oOfX4=',
    privateKey: 'private_tpq2+/lOYMexqf16jLtoVK/k+UM='
});

// Middleware para los headers
app.use((req, res, next) => {
    res.header("Access-Control-Allow-Origin", "*");
    res.header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
    next();
});

app.get('/auth', (req, res) => {
    var result = imagekit.getAuthenticationParameters();
    res.send(result);
});

app.listen(3001, () => {
    console.log("Live on port 3001");
});

