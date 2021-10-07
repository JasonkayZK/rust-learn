use chrono::NaiveDateTime;
use rbatis::core::Error;
use rbatis::crud::CRUD;
use rbatis::executor::Executor;
use rbatis::rbatis::Rbatis;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

pub async fn init_db() {
    RB.link("mysql://root:123456@localhost:3306/test")
        .await
        .unwrap();
}

#[crud_table(table_name: "pets" | table_columns: "id,name,age,photo,ctime,utime")]
#[derive(Clone, Debug)]
pub struct Pet {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub age: Option<i64>,
    pub photo: Option<String>,
    pub ctime: Option<NaiveDateTime>,
    pub utime: Option<NaiveDateTime>,
}

pub async fn create_pet(pet: &Pet) -> Result<i64, Error> {
    return match RB.save(&pet, &[]).await {
        Ok(res) => {
            println!(
                "last_insert_id: {}, rows_affected: {}",
                res.last_insert_id.unwrap(),
                res.rows_affected
            );
            Ok(res.last_insert_id.unwrap())
        }
        Err(e) => Err(e),
    };
}

pub async fn remove_pet_by_id(id: &str) -> Result<u64, Error> {
    RB.remove_by_wrapper::<Pet>(RB.new_wrapper().eq("id", id))
        .await
}

pub async fn update_pet_by_id(pet: &Pet) -> Result<(), Error> {
    return match RB.update_by_column("id", pet).await {
        Ok(rows_affected) => {
            println!("rows_affected: {}", rows_affected);
            Ok(())
        }
        Err(e) => Err(e),
    };
}

pub async fn count_pet() -> Result<i64, Error> {
    RB.fetch("SELECT count(1) FROM pets;", vec![]).await
}

#[py_sql(RB, "select * from pets where id = #{id}")]
pub async fn get_pet_by_id(id: &str) -> Option<Pet> {
    todo!()
}

pub async fn get_pet_list() -> Result<Vec<Pet>, Error> {
    RB.fetch_list().await
}

#[cfg(not(test))]
// #[cfg(test)]
mod tests {
    use crate::pet::{
        count_pet, create_pet, get_pet_by_id, get_pet_list, init_db, remove_pet_by_id,
        update_pet_by_id, Pet,
    };
    use chrono::NaiveDateTime;
    use rbatis::core::value::DateTimeNow;

    #[tokio::test]
    async fn ping_test() {
        init_db().await;
    }

    #[tokio::test]
    async fn create_pet_test() {
        init_db().await;
        let last_id = create_pet(&Pet {
            id: Option::Some(11),
            name: Option::Some(String::from("tester")),
            age: Option::Some(21),
            photo: Option::Some(String::from("https://photo.com/")),
            ctime: Some(NaiveDateTime::now()),
            utime: None,
        })
        .await
        .unwrap();
        assert!(last_id > 0);
        println!("{}", last_id);
    }

    #[tokio::test]
    async fn update_pet_by_id_test() {
        init_db().await;
        let res = update_pet_by_id(&Pet {
            id: Option::Some(11),
            name: Option::Some(String::from("tester-updated")),
            age: Option::Some(21),
            photo: Option::Some(String::from("https://photo.com/")),
            ctime: None,
            utime: None,
        })
        .await
        .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn get_pet_by_id_test() {
        init_db().await;
        let res = get_pet_by_id("11").await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn get_pet_list_test() {
        init_db().await;
        let res = get_pet_list().await.unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn count_pet_test() {
        init_db().await;
        let count = count_pet().await.unwrap();
        assert!(count > 0);
        println!("{}", count);
    }

    #[tokio::test]
    async fn delete_pet_by_id_test() {
        init_db().await;
        let res = remove_pet_by_id("11").await;
        println!("{:?}", res);
    }
}
