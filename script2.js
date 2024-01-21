async function loginUser(event) {
    event.preventDefault();

    const email = document.getElementById('email').value;
    const password = document.getElementById('password').value;
    const phone = document.getElementById('phone').value;

    try {
        const response = await fetch('http://localhost:3000/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password, phone }),
        });

        const data = await response.json();

        if (response.ok) {
            alert(data.message);
            // Redirect to the home page or perform other actions as needed
        } else {
            alert(data.message);
        }
    } catch (error) {
        console.error('Error during login:', error.message);
        alert('An error occurred. Please try again.');
    }
}
