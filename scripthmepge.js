document.addEventListener('DOMContentLoaded', async () => {
    try {
        const response = await fetch('http://localhost:3000/posts', {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        const data = await response.json();

        if (response.ok) {
            displayPosts(data.posts);
        } else {
            alert('Failed to fetch posts. Please try again.');
        }
    } catch (error) {
        console.error('Error fetching posts:', error.message);
        alert('An error occurred while fetching posts. Please try again.');
    }
});

function displayPosts(posts) {
    const postsContainer = document.getElementById('posts-container');

    posts.forEach(post => {
        const postElement = document.createElement('div');
        postElement.className = 'post';

        const contentParagraph = document.createElement('p');
        contentParagraph.className = 'post-content';
        contentParagraph.textContent = post.content;
        postElement.appendChild(contentParagraph);

        const likeButton = document.createElement('button');
        likeButton.className = 'like-button';
        likeButton.textContent = 'Like';
        likeButton.addEventListener('click', function () {
            likePost(post.id);
        });
        postElement.appendChild(likeButton);

        postsContainer.appendChild(postElement);
    });
}

async function likePost(postId) {
    try {
        const response = await fetch(`http://localhost:3000/posts/${postId}/like`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        const data = await response.json();

        if (response.ok) {
            alert(data.message);
            // Refresh the posts or update the UI as needed
        } else {
            alert(data.message);
        }
    } catch (error) {
        console.error('Error liking post:', error.message);
        alert('An error occurred while liking the post. Please try again.');
    }
}
