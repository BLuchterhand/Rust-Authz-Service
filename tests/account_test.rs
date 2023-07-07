#[cfg(test)]
mod tests {
    use postgres::NoTls;
    use project_restful::internal::account::{
        store::DBClient, 
        authorization::{
            authorization_client::AuthorizationClient,
            LoginRequest,
            SignupRequest,
        },
    };
    use tonic::{Request, Status};

#[tokio::test]
async fn test_login() -> Result<(), Box<dyn std::error::Error>> {
    let db_client = {
        let (client, connection) = tokio_postgres::connect("postgresql://postgres:postgres@localhost:5432/postgres", NoTls).await?;
        let db_client = DBClient::new(client);
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        db_client
    };

    let _result = db_client.client.execute(
            "INSERT INTO public.accounts (id, email, password) VALUES($1, $2, $3)",
            &[&10001i32, &"test1@gmail.com", &"$argon2id$v=19$m=19456,t=2,p=1$OZvRCPdpJUHXTly354KWHQ$LWpd6Bt7bqpAGKea0yhwU01UF5cdYUnkYY4xflkGRe4"],
        ).await?;

    let mut client = AuthorizationClient::connect("http://0.0.0.0:40130").await?;

    let request = Request::<LoginRequest>::new(LoginRequest {
        email: "test1@gmail.com".to_string(),
        password: "Hello".to_string(),
    });

    let response = client
        .login(request)
        .await
        .map_err(|err| Status::unknown(err.to_string()))?;

    let _result = db_client.client.execute(
        "delete from public.accounts where id = $1",
            &[&10001i32],
    ).await?;

    assert_eq!(
        response.into_inner().status,
        true
    );
    Ok(())
    }

#[tokio::test]
async fn test_signup() -> Result<(), Box<dyn std::error::Error>> {
    let db_client = {
        let (client, connection) = tokio_postgres::connect("postgresql://postgres:postgres@localhost:5432/postgres", NoTls).await?;
        let db_client = DBClient::new(client);
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        db_client
    };

    let mut client = AuthorizationClient::connect("http://0.0.0.0:40130").await?;

    let request = Request::<SignupRequest>::new(SignupRequest {
        email: "test2@gmail.com".to_string(),
        password: "4321".to_string(),
    });

    let response = client
        .signup(request)
        .await
        .map_err(|err| Status::unknown(err.to_string()))?;

    assert_eq!(
        response.into_inner().message,
        "Your account has been created!".to_string()
    );

    // let request_two = Request::<SignupRequest>::new(SignupRequest {
    //     email: "test2@gmail.com".to_string(),
    //     password: "4321".to_string(),
    // });

    // let response = client
    //     .signup(request_two)
    //     .await
    //     .map_err(|err| Status::unknown(err.to_string()))?;

    // assert_eq!(
    //     response.into_inner().message,
    //     "That email is already linked to an account.".to_string()
    // );

    let _result = db_client.client.execute(
        "delete from public.accounts where email = $1",
            &[&"test2@gmail.com"],
    ).await?;

    Ok(())
    }
}
