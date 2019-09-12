#[derive(Serialize, Default, Debug, Clone, Deserialize)]
pub struct Student {
    pub id: i32,
    pub street: String,
    pub city: String,
    pub sendstatus: String,
    pub datatype: String,
    pub ops: String,
    pub age: i32,
    pub clientid: i32,
    pub indexid: i32,
}