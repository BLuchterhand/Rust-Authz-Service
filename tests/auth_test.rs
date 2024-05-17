#[cfg(test)]
mod tests {
    use postgres::NoTls;
    use project_restful::internal::account::{
        store::DBClient, 
        authorization::{
            authorization_client::AuthorizationClient,
            SignupRequest,
        },
    };
    use tonic::{Request, Status};

    #[tokio::test]
    async fn test_signup_already_exists_error() -> Result<(), Box<dyn std::error::Error>> {
        let db_client = {
            let (client, connection) = tokio_postgres::connect("postgresql://postgres:postgres@localhost:5432/postgres", NoTls).await?;
            let db_client = DBClient::new(client);
            tokio::spawn(async move {
                connection.await.expect("Failed to finish connection");
            });
            db_client
        };

        let _result = db_client.client.execute(
            "INSERT INTO public.accounts (id, email, password) VALUES($1, $2, $3)",
            &[&10003i32, &"test3@gmail.com", &"$argon2id$v=19$m=19456,t=2,p=1$OZvRCPdpJUHXTly354KWHQ$LWpd6Bt7bqpAGKea0yhwU01UF5cdYUnkYY4xflkGRe4"],
        ).await?;

        let mut client = AuthorizationClient::connect("http://0.0.0.0:40130").await?;

        let request = Request::<SignupRequest>::new(SignupRequest {
            email: "test3@gmail.com".to_string(),
            password: "4321".to_string(),
        });

        let response = client
            .signup(request)
            .await
            .map_err(|err| Status::unknown(err.to_string()))?;

        assert_eq!(
            response.into_inner().message,
            "That email is already linked to an account.".to_string()
        );

        let _result = db_client.client.execute(
            "delete from public.accounts where email = $1",
                &[&"test3@gmail.com"],
        ).await?;

        Ok(())
    }
}
