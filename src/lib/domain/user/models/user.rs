#[derive(Debug)]
pub struct User {
    firstname: First_name,
    lastname: Option<Last_name>
}

#[derive(Debug)]
pub struct First_name(String);

#[derive(Debug)]
pub struct Last_name(String);
