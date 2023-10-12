mod auth;
use actix_web::{web, HttpResponse, Responder, HttpServer, App, ResponseError, get};
use auth::config;
use actix_cors::Cors;
use actix_web::dev::Service;
use actix_files::NamedFile;

async fn index() -> impl Responder {
    let html = r#"<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>User Registration</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
        }

        .container {
            width: 300px;
            padding: 20px;
            border: 1px solid #ccc;
            border-radius: 5px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        }

        .input-group {
            margin-bottom: 20px;
            margin-right: 20px;
        }

        .input-group input {
            width: 100%;
            padding: 10px;
            font-size: 16px;
            border: 1px solid #ccc;
            border-radius: 5px;
        }

        .btn-primary {
            background-color: #007bff;
            color: #fff;
            padding: 10px 20px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }

        .btn-primary:hover {
            background-color: #0056b3;
        }
    </style>
</head>

<body>
<div class="container">
    <h2>User Registration</h2>
    <div class="input-group">
        <label for="username">Username:</label>
        <input type="text" id="username" placeholder="Enter your username">
    </div>
    <div class="input-group">
        <label for="password">Password:</label>
        <input type="password" id="password" placeholder="Enter your password">
    </div>
    <button class="btn-primary" onclick="registerUser()">Register</button>
</div>

<script>
    function registerUser() {
        const username = document.getElementById('username').value;
        const password = document.getElementById('password').value;

        // Perform validation if needed

        // Send the username and password to the server for registration
        // For example, using fetch or XMLHttpRequest to make a POST request to the server

        // Example using fetch (you need to replace the URL with your server endpoint)

        fetch('http://localhost:8080/auth/register', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            username: username,
            password: password
          })
        })
        .then(response => response.json())
        .then(data => {
          // Handle the response from the server
          console.log(data);
        })
        .catch(error => {
          // Handle errors
          if (error.response.status === 401) {
              console.log("User already exists");
          } else {
              console.log("User registered successfully");
          }
        });
    }
</script>
</body>

</html>
"#;
    HttpResponse::Ok().content_type("text/html").body(html)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .wrap_fn(|req, srv| {
                println!("got it {} {}", req.method(), req.uri());
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(config)
            .route("/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}