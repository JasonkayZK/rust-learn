use anyhow::Result;
use zenode::{field, field_def, Operator};
use zenode::FieldType::*;

#[tokio::main]
async fn main() -> Result<()> {
    // create an Operator
    let op = Operator::default();

    // create a schema
    let schema_name = "t_person";
    let id = op.create_schema(
        schema_name,
        "A person schema",
        &mut [
            field_def("name", Str),
            field_def("age", Int),
        ],
    ).await.unwrap();

    // generate schema_id
    let schema_id = format!("{}_{}", schema_name, id);

    // create an instance
    let instance_id = op.create_instance(&schema_id, &mut [
        field("name", "a"),
        field("age", "12"),
    ]).await.unwrap();

    // update the instance
    let _update_id = op.update_instance(
        &schema_id,
        &instance_id,
        &mut [
            field("name", "ab")
        ]).await.unwrap();

    // finally delete the instance
    // let _delete_id = op.delete_instance(&schema_id, &update_id).await.unwrap();

    Ok(())
}
