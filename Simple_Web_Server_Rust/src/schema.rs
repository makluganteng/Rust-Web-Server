diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}