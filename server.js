const express = require('express');
const bodyParser = require('body-parser');

const app = express();
const PORT = 3000;

// Middleware to parse JSON in the request body
app.use(bodyParser.json());

// In-memory user storage (replace this with a database in production)
const users = [
    { id: 1, email: 'user1@example.com', password: 'password1', phone: '1234567890' },
    { id: 2, email: 'user2@example.com', password: 'password2', phone: '9876543210' },
];

// User authentication route
app.post('/login', (req, res) => {
    const { email, password, phone } = req.body;

    const authenticatedUser = users.find(user => user.email === email && user.password === password && user.phone === phone);

    if (authenticatedUser) {
        res.status(200).json({ message: 'Login successful!', user: authenticatedUser });
    } else {
        res.status(401).json({ message: 'Invalid credentials. Please try again.' });
    }
});

app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});
