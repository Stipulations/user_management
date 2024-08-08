# User Management System

Welcome to the User Management System project! This is an open-source initiative aimed at creating a robust system for managing user accounts with authentication and role-based access control. 

## Features

Here's what we're planning to build:

- **User Authentication**
  - Login
  - Logout
  - Registration

- **User Roles and Permissions**
  - Admin
    * Manage all other users
    * Access all routes
    * Perform administrative tasks
  - Moderator
    * Moderate user-generated content
    * Access to certain administrative functions
  - User
    * Access user-specific routes
    * Manage own profile

- **Profile Management**
  - View Profile
  - Update Profile

- **Role-based Access Control (RBAC)**
  - Ensure users have appropriate permissions based on their role

## Technologies

We're using a modern tech stack to build this system:

- Rust
- Actix-web
- SQLite
- jsonwebtoken

## Routes

### Authentication

- **POST `/login`**: Authenticate user
- **POST `/logout`**: Invalidate session
- **POST `/register`**: Create a new account

### User Management

- **GET `/users`**: List all users (admin only)
- **GET `/users/{id}`**: Get user details (admin only)
- **PUT `/users/{id}`**: Update user details (admin only)
- **DELETE `/users/{id}`**: Delete a user (admin only)

### Profile

- **GET `/profile`**: Get logged-in user's profile
- **PUT `/profile`**: Update logged-in user's profile

## TODO

Here's what we need to do:

### Authentication

- [x] Implement user login functionality
- [ ] Implement user logout functionality
- [ ] Implement user registration functionality

### User Management

- [ ] Create functionality to list all users (admin only)
- [ ] Develop a feature to get details of a specific user (admin only)
- [ ] Implement user detail update functionality (admin only)
- [ ] Implement user deletion functionality (admin only)

### Profile

- [ ] Create a feature to get the logged-in user's profile
- [ ] Implement profile update functionality

### Blacklisting
TBD

### Sessions
TBD

### Frontend via VueJS
TBD

## Contributing

We welcome contributions! If you have ideas for improvements or find any bugs, please open an issue or submit a pull request. Let's build something great together!
