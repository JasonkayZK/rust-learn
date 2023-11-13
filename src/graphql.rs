use graphql_client::GraphQLQuery;
use p2panda_rs::operation::OperationId;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/all_schema_field.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct AllSchemaFieldDefinitions;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/next_args.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct NextArgs;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/publish.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct Publish;

pub type DocumentId = p2panda_rs::document::DocumentId;

pub type DocumentViewId = String;

pub type EntryHash = p2panda_rs::hash::Hash;

pub type LogId = p2panda_rs::entry::LogId;

pub type PublicKey = p2panda_rs::identity::PublicKey;

pub type SeqNum = p2panda_rs::entry::SeqNum;

pub type EncodedEntry = p2panda_rs::entry::EncodedEntry;

pub type EncodedOperation = p2panda_rs::operation::EncodedOperation;
