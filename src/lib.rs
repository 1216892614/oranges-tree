#[cfg(test)]
mod test {
    use crate::prisma;
    use uuid::Uuid;

    #[tokio::test]
    async fn query() {
        let client = prisma::PrismaClient::_builder().build().await.unwrap();

        let ans = client
            ._batch((
                client
                    .org()
                    .find_many(Vec::from([prisma::org::pos::some(Vec::from([
                        prisma::pos::tags::every(Vec::from([prisma::pos_tag::name::equals(
                            "CEO".to_string(),
                        )])),
                    ]))])),
                client
                    .employee()
                    .find_first(Vec::from([prisma::employee::pos::every(Vec::from([
                        prisma::pos::tags::every(Vec::from([prisma::pos_tag::name::equals(
                            "CEO".to_string(),
                        )])),
                    ]))])),
            ))
            .await
            .unwrap();

        println!("{ans:#?}");
    }

    #[tokio::test]
    async fn create() {
        let client = prisma::PrismaClient::_builder().build().await.unwrap();

        let a_id = Uuid::new_v4().to_string();
        let b_id = Uuid::new_v4().to_string();
        let nerd_id = Uuid::new_v4().to_string();
        let orange_id = Uuid::new_v4().to_string();
        let ceo_id = Uuid::new_v4().to_string();
        let nerd_ceo_id = Uuid::new_v4().to_string();
        let orange_ceo_id = Uuid::new_v4().to_string();

        let ans = client
            ._batch((
                client.org().create(
                    "A".to_string(),
                    Vec::from([prisma::org::id::set(a_id.clone())]),
                ),
                client.org().create(
                    "B".to_string(),
                    Vec::from([prisma::org::id::set(b_id.clone())]),
                ),
                client.employee().create(
                    "Nerd".to_string(),
                    Vec::from([prisma::employee::id::set(nerd_id.clone())]),
                ),
                client.employee().create(
                    "Orange".to_string(),
                    Vec::from([prisma::employee::id::set(orange_id.clone())]),
                ),
                client.pos_tag().create(
                    "CEO".to_string(),
                    Vec::from([prisma::pos_tag::id::set(ceo_id.clone())]),
                ),
                client.pos().create_unchecked(
                    a_id,
                    nerd_id,
                    Vec::from([prisma::pos::id::set(nerd_ceo_id.clone())]),
                ),
                client.pos().update(
                    prisma::pos::id::equals(nerd_ceo_id),
                    Vec::from([prisma::pos::tags::connect(Vec::from([
                        prisma::pos_tag::id::equals(ceo_id.clone()),
                    ]))]),
                ),
                client.pos().create_unchecked(
                    b_id,
                    orange_id,
                    Vec::from([prisma::pos::id::set(orange_ceo_id.clone())]),
                ),
                client.pos().update(
                    prisma::pos::id::equals(orange_ceo_id),
                    Vec::from([prisma::pos::tags::connect(Vec::from([
                        prisma::pos_tag::id::equals(ceo_id),
                    ]))]),
                ),
            ))
            .await
            .unwrap();

        println!("{ans:#?}");
    }
}

mod prisma {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
