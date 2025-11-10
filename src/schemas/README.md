# Schemas (DTOs)

DTOs define the shape of data coming into and going out of the application.

These API requests may not have the same fields as the database models. 

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
```

Notice the above does not have all of the fields of the example user model shown in the models `README.md` file.
