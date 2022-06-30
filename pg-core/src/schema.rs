table! {
    cart_items (id) {
        id -> Int4,
        cart_id -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
    }
}

table! {
    carts (id) {
        id -> Int4,
        custoemr_id -> Nullable<Int4>,
    }
}

table! {
    cashers (id) {
        id -> Int4,
        wallet -> Nullable<Int4>,
    }
}

table! {
    customers (id) {
        id -> Int4,
        wallet -> Nullable<Int4>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        category -> Nullable<Int4>,
        price -> Nullable<Int4>,
    }
}

joinable!(cart_items -> carts (id));
joinable!(cart_items -> products (id));
joinable!(carts -> customers (id));

allow_tables_to_appear_in_same_query!(
    cart_items,
    carts,
    cashers,
    customers,
    posts,
    products,
);
