use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub age: &'a i32,
    pub email: &'a str,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub email: String,
}