Create Table users (
    user_id Integer Primary key,
    name Text,
    country Text
);

CREATE TABLE orders (
    order_id Integer Primary key,
    user_id Integer,
    amount Integer,
    order_date TEXT
);

CREATE TABLE products (
    product_id Integer Primary key,
    name Text,
    category Text
);