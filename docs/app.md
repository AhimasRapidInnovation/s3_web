# Application structure

|
|------ Auth (Login and signup)
|      |
|      |___ Once logged in redirect to Home Page
|
|------- S3 service (Requires Authentication)

## Handling the error

`ResponseError` is a trait that is exposed by `actix_web`  
It needs to method  
`fn status_code(&self) -> StatusCode;` and `fn error_response(&self) -> Response;`

rust's `Error` exposes two method

1) description

2) cause

### Using [thiserror](https://docs.rs/thiserror/latest/thiserror/)

A Display impl is generated for your error if you provide #[error("...")] on variant(enum) or on struct 

* #[error("{var}")] ⟶ write!("{}", self.var)

* #[error("{0}")] ⟶ write!("{}", self.0)

* #[error("{var:?}")] ⟶ write!("{:?}", self.var)
* #[error("{0:?}")] ⟶ write!("{:?}", self.0)

From impl is generated for each variant containing a #[from] attribute.
