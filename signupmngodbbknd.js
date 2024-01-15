const express = require('express');
const bodyParser = require('body-parser');
const mongoose = require('mongoose');

const app = express();
const port = 3000;

// Connect to MongoDB
mongoose.connect('mongodb://localhost:27017/spydr-social', { useNewUrlParser: true, useUnifiedTopology: true });

// Define a User schema
const userSchema = new mongoose.Schema({
  email: String,
  password: String,
  phoneNumber: String,
});

const User = mongoose.model('User', userSchema);

app.use(bodyParser.urlencoded({ extended: true }));

app.get('/', (req, res) => {
  res.send('Sign Up for SPYDR Social');
});

app.get('/signup', (req, res) => {
  res.sendFile(__dirname + '/sign-up.html');
});

app.post('/signup', async (req, res) => {
  const { email, password, phoneNumber } = req.body;

  try {
    // Create a new user
    const newUser = new User({ email, password, phoneNumber });
    
    // Save the user to the database
    await newUser.save();
    
    res.send('Signup successful!');
  } catch (error) {
    console.error(error);
    res.status(500).send('Error during signup');
  }
});

app.listen(port, () => {
  console.log(`Server is running at http://localhost:${port}`);
});
