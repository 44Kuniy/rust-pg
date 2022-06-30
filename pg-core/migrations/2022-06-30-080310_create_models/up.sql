CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL default '',
  category INTEGER,
  price INTEGER
);

CREATE TABLE cashers (id SERIAL PRIMARY KEY, wallet INTEGER);

CREATE TABLE customers (id SERIAL PRIMARY KEY, wallet INTEGER);

CREATE TABLE carts (
  id SERIAL PRIMARY KEY,
  custoemr_id INTEGER,
  CONSTRAINT fk_carts_custoemr_id FOREIGN KEY (id) REFERENCES customers (id)
);

CREATE TABLE cart_items (
  id SERIAL PRIMARY KEY,
  cart_id INTEGER,
  product_id INTEGER,
  CONSTRAINT fk_cart_items_cart_id FOREIGN KEY (id) REFERENCES carts (id),
  CONSTRAINT fk_cart_items_product_id FOREIGN KEY (id) REFERENCES products (id)
);