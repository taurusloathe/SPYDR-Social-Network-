app.post('/register', async (req, res) => {
  try {
    // Your registration logic here
    // ...
  } catch (error) {
    console.error(error.message);
    res.status(500).send('Registration failed');
  }
});
