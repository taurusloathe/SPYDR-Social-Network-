const express = require('express');
const bodyParser = require('body-parser');

const app = express();
const port = 3000;

app.use(bodyParser.urlencoded({ extended: true }));

app.get('/', (req, res) => {
  res.send('Signup for SPYDR Social');
});

app.get('/signup', (req, res) => {
  res.sendFile(__dirname + '/signup.html');
});

app.post('/signup', (req, res) => {
  const { email, password, phoneNumber } = req.body;

  // You'd typically store this information in a database
  console.log(`New user signed up:\nEmail: ${email}\nPassword: ${password}\nPhone Number: ${phoneNumber}`);

  res.send('Signup successful!');
});

app.listen(port, () => {
  console.log(`Server is running at http://localhost:${port}`);
});
